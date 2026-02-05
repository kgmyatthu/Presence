use std::path::PathBuf;

use crate::core::{AttendanceReport, ReportFormat};

#[derive(Debug, Clone)]
pub enum Message {
    DirectoryChanged(String),
    PickDirectory,
    DirectoryPicked(Option<PathBuf>),
    PickFile,
    FilePicked(Option<PathBuf>),
    ClassStartChanged(String),
    ClassEndChanged(String),
    LateMinutesChanged(String),
    AbsentMinutesChanged(String),
    TotalPointsChanged(String),
    LatePenaltyChanged(String),
    AbsentPenaltyChanged(String),
    ReportFormatChanged(ReportFormat),
    RunAnalysis,
    AnalysisDone(Result<AttendanceReport, String>),
    SelectStudent(usize),
    ExportReport,
    ReportSaved(Result<PathBuf, String>),
}
