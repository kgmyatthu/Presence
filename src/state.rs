use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use genpdf::{style::{Color, Style}, Element};
use genpdf::elements::PaddedElement;

use crate::core::{
    self, AttendanceConfig, AttendanceReport, ReportFormat, Participant
};

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

pub fn load_attendance(
    directory: PathBuf,
    config: AttendanceConfig,
) -> Result<AttendanceReport, String> {
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

    let mut sessions: Vec<Vec<Participant>> = Vec::new();

    for path in files {
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let data = std::fs::read(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
        let participants = core::parse_participants(&data, extension)?;
        if !participants.is_empty() {
            sessions.push(participants);
        }
    }

    core::generate_report(sessions, config)
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

fn is_attendance_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("csv") | Some("xlsx") | Some("xls")
    )
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

    let mut table = genpdf::elements::TableLayout::new(vec![3, 3, 2, 2, 2, 2, 2]);
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
