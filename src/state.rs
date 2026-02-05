use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use genpdf::{style::{Color, Style}, Element};
use genpdf::elements::PaddedElement;
use calamine::{Data, Reader, Xlsx};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use csv::StringRecord;
use encoding_rs::{UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1252};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct AppState {
    pub directory: String,
    pub class_start: String,
    pub class_end: String,
    pub late_minutes: String,
    pub absent_minutes: String,
    pub total_points: String,
    pub late_penalty: String,
    pub absent_penalty: String,
    pub report_format: ReportFormat,
    pub report: Option<AttendanceReport>,
    pub selected_student: Option<usize>,
    pub status: String,
    pub is_busy: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            directory: String::new(),
            class_start: "13:30".to_string(),
            class_end: "15:00".to_string(),
            late_minutes: "10".to_string(),
            absent_minutes: "30".to_string(),
            total_points: "10".to_string(),
            late_penalty: "0.5".to_string(),
            absent_penalty: "1".to_string(),
            report_format: ReportFormat::Csv,
            report: None,
            selected_student: None,
            status: "Select a directory to begin.".to_string(),
            is_busy: false,
        }
    }

    pub fn to_config(&self) -> AttendanceConfig {
        AttendanceConfig {
            class_start: self.class_start.clone(),
            class_end: self.class_end.clone(),
            late_minutes: self.late_minutes.clone(),
            absent_minutes: self.absent_minutes.clone(),
            total_points: self.total_points.clone(),
            late_penalty: self.late_penalty.clone(),
            absent_penalty: self.absent_penalty.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttendanceConfig {
    pub class_start: String,
    pub class_end: String,
    pub late_minutes: String,
    pub absent_minutes: String,
    pub total_points: String,
    pub late_penalty: String,
    pub absent_penalty: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Csv,
    Txt,
    Pdf,
}

impl ReportFormat {
    pub const ALL: [ReportFormat; 3] = [ReportFormat::Csv, ReportFormat::Txt, ReportFormat::Pdf];
}

impl std::fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            ReportFormat::Csv => "CSV",
            ReportFormat::Txt => "Text",
            ReportFormat::Pdf => "PDF",
        };
        write!(f, "{label}")
    }
}

#[derive(Debug, Clone)]
pub struct AttendanceReport {
    pub students: Vec<StudentRecord>,
    pub sessions: usize,
    pub total_points: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct StudentRecord {
    pub name: String,
    pub surname: String,
    pub id: String,
    pub email: String,
    pub normal: u32,
    pub late: u32,
    pub absent: u32,
    pub score: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttendanceStatus {
    Normal,
    Late,
    Absent,
}

#[derive(Debug, Clone)]
struct Participant {
    name: String,
    surname: String,
    id: String,
    email: String,
    first_join: NaiveDateTime,
}

#[derive(Debug, Clone)]
struct ConfigValues {
    class_start: NaiveTime,
    late_minutes: i64,
    absent_minutes: i64,
    total_points: f32,
    late_penalty: f32,
    absent_penalty: f32,
}

pub fn load_attendance(
    directory: PathBuf,
    config: AttendanceConfig,
) -> Result<AttendanceReport, String> {
    let config = parse_config(config)?;
    let mut files: Vec<PathBuf> = if directory.is_file() {
        if is_attendance_file(&directory) {
            vec![directory]
        } else {
            return Err("Selected file is not a supported CSV/XLSX attendance export.".to_string());
        }
    } else if directory.is_dir() {
        std::fs::read_dir(&directory)
            .map_err(|error| format!("Failed to read directory: {error}"))?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| is_attendance_file(path))
            .collect()
    } else {
        return Err("Please select a valid directory or attendance file.".to_string());
    };
    files.sort();

    if files.is_empty() {
        return Err("No attendance CSV/XLSX files found in the directory.".to_string());
    }

    let mut students: HashMap<String, StudentRecord> = HashMap::new();
    let mut sessions_processed = 0usize;

    for path in files {
        let participants = read_participants(&path)?;
        if participants.is_empty() {
            continue;
        }

        let session_date = participants
            .first()
            .map(|participant| participant.first_join.date())
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let class_start = NaiveDateTime::new(session_date, config.class_start);

        let mut session_keys: HashSet<String> = HashSet::new();
        let mut session_status: HashMap<String, AttendanceStatus> = HashMap::new();

        for participant in participants {
            let key = build_key(&participant);
            let status = classify_attendance(&participant, class_start, &config);
            session_keys.insert(key.clone());
            session_status.insert(key.clone(), status);

            let record = students.entry(key).or_insert_with(|| StudentRecord {
                name: participant.name.clone(),
                surname: participant.surname.clone(),
                id: participant.id.clone(),
                email: participant.email.clone(),
                normal: 0,
                late: 0,
                absent: sessions_processed as u32,
                score: 0.0,
            });

            apply_status(record, status);
        }

        for (key, record) in students.iter_mut() {
            if !session_keys.contains(key) {
                record.absent += 1;
            }
        }

        sessions_processed += 1;
    }

    for record in students.values_mut() {
        record.score = calculate_score(record, &config);
    }

    let mut students: Vec<StudentRecord> = students.into_values().collect();
    students.sort_by(|a, b| a.surname.cmp(&b.surname).then(a.name.cmp(&b.name)));

    Ok(AttendanceReport {
        students,
        sessions: sessions_processed,
        total_points: config.total_points,
    })
}

pub fn save_report(report: AttendanceReport, format: ReportFormat) -> Result<PathBuf, String> {
    let file = rfd::FileDialog::new()
        .save_file()
        .ok_or_else(|| "Save cancelled. Please choose a file path to export.".to_string())?;
    match format {
        ReportFormat::Csv => write_csv(&file, &report)?,
        ReportFormat::Txt => write_text(&file, &report)?,
        ReportFormat::Pdf => write_pdf(&file, &report)?,
    }
    Ok(file)
}

fn parse_config(config: AttendanceConfig) -> Result<ConfigValues, String> {
    let class_start = parse_time(&config.class_start)?;
    let class_end = parse_time(&config.class_end)?;
    if class_end <= class_start {
        return Err("Class end time must be after the start time.".to_string());
    }
    let late_minutes = config
        .late_minutes
        .trim()
        .parse::<i64>()
        .map_err(|_| "Late minutes must be a number.".to_string())?;
    let absent_minutes = config
        .absent_minutes
        .trim()
        .parse::<i64>()
        .map_err(|_| "Absent minutes must be a number.".to_string())?;
    let total_points = parse_float(&config.total_points, "Total points")?;
    let late_penalty = parse_float(&config.late_penalty, "Late penalty")?;
    let absent_penalty = parse_float(&config.absent_penalty, "Absent penalty")?;
    Ok(ConfigValues {
        class_start,
        late_minutes,
        absent_minutes,
        total_points,
        late_penalty,
        absent_penalty,
    })
}

fn parse_time(input: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(input.trim(), "%H:%M")
        .map_err(|_| format!("Invalid time format: {input}. Use HH:MM."))
}

fn parse_float(input: &str, label: &str) -> Result<f32, String> {
    input
        .trim()
        .parse::<f32>()
        .map_err(|_| format!("{label} must be a number."))
}

fn is_attendance_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("csv") | Some("xlsx") | Some("xls")
    )
}

fn read_participants(path: &Path) -> Result<Vec<Participant>, String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("csv") => read_csv_participants(path),
        Some("xlsx") | Some("xls") => read_excel_participants(path),
        _ => Ok(Vec::new()),
    }
}

fn read_csv_participants(path: &Path) -> Result<Vec<Participant>, String> {
    let (contents, delimiter) = read_csv_text(path)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(delimiter)
        .flexible(true)
        .from_reader(contents.as_bytes());
    let mut header: Option<Vec<String>> = None;
    let mut participants: HashMap<String, Participant> = HashMap::new();

    for result in reader.records() {
        let record = result.map_err(|error| format!("Failed to read CSV: {error}"))?;
        if header.is_none() && record.iter().any(|cell| cell == "Name") {
            header = Some(record.iter().map(|cell| cell.trim().to_string()).collect());
            continue;
        }
        if let Some(header) = &header {
            let participant = parse_participant_row(header, &record)?;
            if let Some(participant) = participant {
                let key = build_key(&participant);
                participants
                    .entry(key)
                    .and_modify(|existing| {
                        if participant.first_join < existing.first_join {
                            *existing = participant.clone();
                        }
                    })
                    .or_insert(participant);
            }
        }
    }

    Ok(participants.into_values().collect())
}

fn read_csv_text(path: &Path) -> Result<(String, u8), String> {
    let bytes = std::fs::read(path).map_err(|error| format!("Failed to read CSV: {error}"))?;
    let (decoded, _encoding) = if bytes.starts_with(&[0xFF, 0xFE]) {
        UTF_16LE.decode_with_bom_removal(&bytes)
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        UTF_16BE.decode_with_bom_removal(&bytes)
    } else if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        UTF_8.decode_with_bom_removal(&bytes)
    } else {
        match String::from_utf8(bytes.clone()) {
            Ok(value) => return Ok((value, detect_delimiter(&bytes))),
            Err(_) => WINDOWS_1252.decode_without_bom_handling(&bytes),
        }
    };
    let delimiter = detect_delimiter(&bytes);
    Ok((decoded.into_owned(), delimiter))
}

fn detect_delimiter(bytes: &[u8]) -> u8 {
    let sample = String::from_utf8_lossy(bytes);
    let line = sample
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default();
    let comma = line.matches(',').count();
    let tab = line.matches('\t').count();
    let semicolon = line.matches(';').count();
    if tab >= comma && tab >= semicolon {
        b'\t'
    } else if semicolon > comma {
        b';'
    } else {
        b','
    }
}

fn read_excel_participants(path: &Path) -> Result<Vec<Participant>, String> {
    let mut workbook: Xlsx<_> = calamine::open_workbook(path)
        .map_err(|error| format!("Failed to open {path:?}: {error}"))?;
    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or_else(|| "Excel file is missing sheets.".to_string())?;
    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|error| format!("Failed to read Excel sheet: {error}"))?;
    let mut header: Option<Vec<String>> = None;
    let mut participants: HashMap<String, Participant> = HashMap::new();

    for row in range.rows() {
        let record: Vec<String> = row.iter().map(cell_to_string).collect();
        let record = StringRecord::from(record);
        if header.is_none() && record.iter().any(|cell| cell == "Name") {
            header = Some(record.iter().map(|cell| cell.trim().to_string()).collect());
            continue;
        }
        if let Some(header) = &header {
            let participant = parse_participant_row(header, &record)?;
            if let Some(participant) = participant {
                let key = build_key(&participant);
                participants
                    .entry(key)
                    .and_modify(|existing| {
                        if participant.first_join < existing.first_join {
                            *existing = participant.clone();
                        }
                    })
                    .or_insert(participant);
            }
        }
    }

    Ok(participants.into_values().collect())
}

fn parse_participant_row(
    header: &[String],
    record: &StringRecord,
) -> Result<Option<Participant>, String> {
    let name_index = header.iter().position(|cell| cell == "Name");
    let join_index = header.iter().position(|cell| cell == "First Join");
    let email_index = header.iter().position(|cell| cell == "Email");
    if name_index.is_none() || join_index.is_none() {
        return Ok(None);
    }
    let name = record
        .get(name_index.unwrap())
        .map(|value| value.trim())
        .unwrap_or_default();
    if name.is_empty() || name == "Name" {
        return Ok(None);
    }
    let join_value = record
        .get(join_index.unwrap())
        .map(|value| value.trim())
        .unwrap_or_default();
    if join_value.is_empty() {
        return Ok(None);
    }
    let email = email_index
        .and_then(|index| record.get(index))
        .map(|value| value.trim().to_string())
        .unwrap_or_default();
    let first_join = parse_datetime(join_value)?;
    let (first, surname) = split_name(name);
    let id = extract_id(&email);
    Ok(Some(Participant {
        name: first,
        surname,
        id,
        email,
        first_join,
    }))
}

fn parse_datetime(input: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(input, "%m/%d/%y, %I:%M:%S %p")
        .or_else(|_| NaiveDateTime::parse_from_str(input, "%m/%d/%Y, %I:%M:%S %p"))
        .map_err(|_| format!("Invalid datetime: {input}"))
}

fn split_name(input: &str) -> (String, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return (String::new(), String::new());
    }
    if parts.len() == 1 {
        return (parts[0].to_string(), String::new());
    }
    let name = parts[0].to_string();
    let surname = parts[1..].join(" ");
    (name, surname)
}

fn extract_id(email: &str) -> String {
    email.split('@').next().unwrap_or_default().to_string()
}

fn build_key(participant: &Participant) -> String {
    if !participant.id.is_empty() {
        participant.id.clone()
    } else if !participant.email.is_empty() {
        participant.email.clone()
    } else {
        format!("{} {}", participant.name, participant.surname)
    }
}

fn classify_attendance(
    participant: &Participant,
    class_start: NaiveDateTime,
    config: &ConfigValues,
) -> AttendanceStatus {
    let delta = participant.first_join - class_start;
    let minutes = delta.num_minutes().max(0);
    if minutes <= config.late_minutes {
        AttendanceStatus::Normal
    } else if minutes <= config.absent_minutes {
        AttendanceStatus::Late
    } else {
        AttendanceStatus::Absent
    }
}

fn apply_status(record: &mut StudentRecord, status: AttendanceStatus) {
    match status {
        AttendanceStatus::Normal => record.normal += 1,
        AttendanceStatus::Late => record.late += 1,
        AttendanceStatus::Absent => record.absent += 1,
    }
}

fn calculate_score(record: &StudentRecord, config: &ConfigValues) -> f32 {
    let deductions =
        (record.late as f32 * config.late_penalty) + (record.absent as f32 * config.absent_penalty);
    let score = config.total_points - deductions;
    score.max(0.0)
}

fn write_csv(path: &Path, report: &AttendanceReport) -> Result<(), String> {
    let mut writer =
        csv::Writer::from_path(path).map_err(|error| format!("Failed to create CSV: {error}"))?;
    writer
        .write_record(["Name", "Surname", "ID", "Normal", "Late", "Absent", "Score"])
        .map_err(|error| format!("Failed to write CSV header: {error}"))?;
    for student in &report.students {
        writer
            .write_record([
                &student.name,
                &student.surname,
                &student.id,
                &student.normal.to_string(),
                &student.late.to_string(),
                &student.absent.to_string(),
                &format!("{:.1}/{:.1}", student.score, report.total_points),
            ])
            .map_err(|error| format!("Failed to write CSV row: {error}"))?;
    }
    writer
        .flush()
        .map_err(|error| format!("Failed to finalize CSV: {error}"))?;
    Ok(())
}

fn write_text(path: &Path, report: &AttendanceReport) -> Result<(), String> {
    let mut file = File::create(path).map_err(|error| format!("Failed to create text: {error}"))?;
    writeln!(file, "Name\tSurname\tID\tNormal\tLate\tAbsent\tScore")
        .map_err(|error| format!("Failed to write text header: {error}"))?;
    for student in &report.students {
        writeln!(
            file,
            "{}\t{}\t{}\t{}\t{}\t{}\t{:.1}/{:.1}",
            student.name,
            student.surname,
            student.id,
            student.normal,
            student.late,
            student.absent,
            student.score,
            report.total_points
        )
        .map_err(|error| format!("Failed to write text row: {error}"))?;
    }
    Ok(())
}

fn write_pdf(path: &Path, report: &AttendanceReport) -> Result<(), String> {
    let final_path = if let Some(extension) = path.extension() {
        if extension != "pdf" {
            path.with_extension("pdf")
        } else {
            path.to_path_buf()
        }
    } else {
        path.with_extension("pdf")
    };

    let font_data = include_bytes!("../assets/fonts/DejaVuSans.ttf");
    let bold_data = include_bytes!("../assets/fonts/DejaVuSans-Bold.ttf");

    let font = genpdf::fonts::FontData::new(font_data.to_vec(), None)
        .map_err(|e| format!("Failed to load font: {}", e))?;
    let bold_font = genpdf::fonts::FontData::new(bold_data.to_vec(), None)
        .map_err(|e| format!("Failed to load bold font: {}", e))?;

    let font_family = genpdf::fonts::FontFamily {
        regular: font.clone(),
        bold: bold_font.clone(),
        italic: font,
        bold_italic: bold_font,
    };
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Attendance Report");

    let blue = Color::Rgb(38, 139, 210);
    let violet = Color::Rgb(108, 113, 196);
    let base01 = Color::Rgb(88, 110, 117);
    let base00 = Color::Rgb(101, 123, 131);

    let mut title = genpdf::elements::Paragraph::new("Attendance Report");
    title.set_alignment(genpdf::Alignment::Center);
    doc.push(title.styled(Style::new().with_color(violet).with_font_size(20).bold()));
    doc.push(genpdf::elements::Break::new(1.0));

    let mut table = genpdf::elements::TableLayout::new(vec![3, 3, 2, 1, 1, 1, 2]);
    table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, true));

    let header_style = Style::new().with_color(blue).bold();

    table
        .row()
        .element(padded_text("Name", header_style))
        .element(padded_text("Surname", header_style))
        .element(padded_text("ID", header_style))
        .element(padded_text("Normal", header_style))
        .element(padded_text("Late", header_style))
        .element(padded_text("Absent", header_style))
        .element(padded_text("Score", header_style))
        .push()
        .map_err(|error| format!("Failed to write PDF header: {error}"))?;
    for (i, student) in report.students.iter().enumerate() {
        let color = if i % 2 == 0 { base01 } else { base00 };
        let row_style = Style::new().with_color(color);
        table
            .row()
            .element(padded_text(student.name.clone(), row_style))
            .element(padded_text(student.surname.clone(), row_style))
            .element(padded_text(student.id.clone(), row_style))
            .element(padded_text(student.normal.to_string(), row_style))
            .element(padded_text(student.late.to_string(), row_style))
            .element(padded_text(student.absent.to_string(), row_style))
            .element(padded_text(format!(
                "{:.1}/{:.1}",
                student.score, report.total_points
            ), row_style))
            .push()
            .map_err(|error| format!("Failed to write PDF row: {error}"))?;
    }
    doc.push(table);
    doc.render_to_file(&final_path)
        .map_err(|error| format!("Failed to write PDF: {error}"))?;
    Ok(())
}

fn padded_text(text: impl Into<String>, style: Style) -> impl Element {
    PaddedElement::new(
        genpdf::elements::Paragraph::new(text.into()).styled(style),
        genpdf::Margins::trbl(2.0, 2.0, 2.0, 2.0),
    )
}

fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::String(value) => value.clone(),
        Data::Float(value) => value.to_string(),
        Data::Int(value) => value.to_string(),
        Data::Bool(value) => value.to_string(),
        Data::DateTime(value) => value.to_string(),
        Data::DateTimeIso(value) => value.to_string(),
        Data::DurationIso(value) => value.to_string(),
        Data::Error(error) => format!("{error:?}"),
        Data::Empty => String::new(),
    }
}
