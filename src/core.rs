use std::collections::{HashMap, HashSet};
use std::io::Cursor;
use calamine::{Data, Reader, Xlsx};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use csv::StringRecord;
use encoding_rs::{UTF_16BE, UTF_16LE, UTF_8, WINDOWS_1252};
use serde::Serialize;

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
pub struct Participant {
    pub name: String,
    pub surname: String,
    pub id: String,
    pub email: String,
    pub first_join: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ConfigValues {
    pub class_start: NaiveTime,
    pub late_minutes: i64,
    pub absent_minutes: i64,
    pub total_points: f32,
    pub late_penalty: f32,
    pub absent_penalty: f32,
}

pub fn parse_config(config: AttendanceConfig) -> Result<ConfigValues, String> {
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

pub fn parse_participants(data: &[u8], extension: &str) -> Result<Vec<Participant>, String> {
    match extension {
        "csv" => parse_csv_participants(data),
        "xlsx" | "xls" => parse_excel_participants(data),
        _ => Ok(Vec::new()),
    }
}

fn parse_csv_participants(data: &[u8]) -> Result<Vec<Participant>, String> {
    let (contents, delimiter) = decode_csv_text(data);
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

fn decode_csv_text(bytes: &[u8]) -> (String, u8) {
    let (decoded, _encoding) = if bytes.starts_with(&[0xFF, 0xFE]) {
        UTF_16LE.decode_with_bom_removal(bytes)
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        UTF_16BE.decode_with_bom_removal(bytes)
    } else if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        UTF_8.decode_with_bom_removal(bytes)
    } else {
        match std::str::from_utf8(bytes) {
            Ok(value) => return (value.to_string(), detect_delimiter(bytes)),
            Err(_) => WINDOWS_1252.decode_without_bom_handling(bytes),
        }
    };
    let delimiter = detect_delimiter(bytes);
    (decoded.into_owned(), delimiter)
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

fn parse_excel_participants(data: &[u8]) -> Result<Vec<Participant>, String> {
    let cursor = Cursor::new(data);
    let mut workbook: Xlsx<_> = Xlsx::new(cursor)
        .map_err(|error| format!("Failed to open Excel data: {error}"))?;
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

pub fn split_name(input: &str) -> (String, String) {
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

pub fn extract_id(email: &str) -> String {
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

pub fn generate_report(
    sessions: Vec<Vec<Participant>>,
    config: AttendanceConfig,
) -> Result<AttendanceReport, String> {
    let config = parse_config(config)?;
    let mut students: HashMap<String, StudentRecord> = HashMap::new();
    let mut sessions_processed = 0usize;

    for participants in sessions {
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
