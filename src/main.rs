mod messages;
mod state;

use std::path::PathBuf;

use iced::alignment::{Horizontal, Vertical};
use iced::mouse;
use iced::widget::canvas::{self, Canvas};
use iced::widget::{
    Column, Space, button, column, container, pick_list, row, scrollable, text, text_input,
};
use iced::{
    Alignment, Application, Color, Command, Element, Length, Pixels, Point, Radians, Rectangle,
    Renderer, Settings, Size, Theme, executor,
};
use messages::Message;
use state::{AppState, AttendanceReport, ReportFormat, StudentRecord};

const NAME_COLUMN_WIDTH: f32 = 150.0;
const SURNAME_COLUMN_WIDTH: f32 = 150.0;
const ID_COLUMN_WIDTH: f32 = 120.0;
const COUNT_COLUMN_WIDTH: f32 = 80.0;
const SCORE_COLUMN_WIDTH: f32 = 100.0;

fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            min_size: Some(Size::new(800.0, 600.0)),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

struct App {
    state: AppState,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                state: AppState::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Presence - Attendance Analyzer".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DirectoryChanged(value) => {
                self.state.directory = value;
                if self.state.directory.trim().is_empty() {
                    self.state.status = "Select a directory to begin.".to_string();
                } else {
                    self.state.status =
                        "Directory selected. Click Generate Report to analyze.".to_string();
                }
                Command::none()
            }
            Message::PickDirectory => Command::perform(pick_directory(), Message::DirectoryPicked),
            Message::DirectoryPicked(path) => {
                if let Some(path) = path {
                    self.state.directory = path.display().to_string();
                    self.state.status =
                        "Directory selected. Click Generate Report to analyze.".to_string();
                }
                Command::none()
            }
            Message::PickFile => Command::perform(pick_file(), Message::FilePicked),
            Message::FilePicked(path) => {
                if let Some(path) = path {
                    self.state.directory = path.display().to_string();
                    self.state.status =
                        "File selected. Click Generate Report to analyze.".to_string();
                }
                Command::none()
            }
            Message::ClassStartChanged(value) => {
                self.state.class_start = value;
                Command::none()
            }
            Message::ClassEndChanged(value) => {
                self.state.class_end = value;
                Command::none()
            }
            Message::LateMinutesChanged(value) => {
                self.state.late_minutes = value;
                Command::none()
            }
            Message::AbsentMinutesChanged(value) => {
                self.state.absent_minutes = value;
                Command::none()
            }
            Message::TotalPointsChanged(value) => {
                self.state.total_points = value;
                Command::none()
            }
            Message::LatePenaltyChanged(value) => {
                self.state.late_penalty = value;
                Command::none()
            }
            Message::AbsentPenaltyChanged(value) => {
                self.state.absent_penalty = value;
                Command::none()
            }
            Message::ReportFormatChanged(format) => {
                self.state.report_format = format;
                Command::none()
            }
            Message::RunAnalysis => {
                self.state.status = "Analyzing attendance files...".to_string();
                self.state.is_busy = true;
                let config = self.state.to_config();
                let directory = PathBuf::from(self.state.directory.clone());
                Command::perform(load_attendance(directory, config), Message::AnalysisDone)
            }
            Message::AnalysisDone(result) => {
                self.state.is_busy = false;
                match result {
                    Ok(report) => {
                        self.state.status = format!(
                            "Loaded {} students across {} sessions.",
                            report.students.len(),
                            report.sessions
                        );
                        self.state.report = Some(report);
                    }
                    Err(error) => {
                        self.state.status = error;
                        self.state.report = None;
                    }
                }
                Command::none()
            }
            Message::SelectStudent(index) => {
                self.state.selected_student = Some(index);
                Command::none()
            }
            Message::ExportReport => {
                if let Some(report) = self.state.report.clone() {
                    self.state.status = "Exporting report...".to_string();
                    self.state.is_busy = true;
                    let format = self.state.report_format;
                    Command::perform(save_report(report, format), Message::ReportSaved)
                } else {
                    self.state.status = "Load attendance data before exporting.".to_string();
                    Command::none()
                }
            }
            Message::ReportSaved(result) => {
                self.state.is_busy = false;
                match result {
                    Ok(path) => {
                        self.state.status = format!("Report saved to {}", path.display());
                    }
                    Err(error) => self.state.status = error,
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input_section = column![
            text("Attendance Input").size(24),
            row![
                text_input("Attendance directory or file", &self.state.directory)
                    .on_input(Message::DirectoryChanged)
                    .size(16)
                    .padding(8)
                    .width(Length::Fill),
                button("Browse Folder").on_press(Message::PickDirectory),
                button("Browse File").on_press(Message::PickFile)
            ]
            .spacing(8),
            row![
                text_input("Class start (HH:MM)", &self.state.class_start)
                    .on_input(Message::ClassStartChanged)
                    .padding(8),
                text_input("Class end (HH:MM)", &self.state.class_end)
                    .on_input(Message::ClassEndChanged)
                    .padding(8),
            ]
            .spacing(12),
            row![
                text_input("Late after minutes", &self.state.late_minutes)
                    .on_input(Message::LateMinutesChanged)
                    .padding(8),
                text_input("Absent after minutes", &self.state.absent_minutes)
                    .on_input(Message::AbsentMinutesChanged)
                    .padding(8),
            ]
            .spacing(12),
            row![
                text_input("Total points", &self.state.total_points)
                    .on_input(Message::TotalPointsChanged)
                    .padding(8),
                text_input("Late penalty", &self.state.late_penalty)
                    .on_input(Message::LatePenaltyChanged)
                    .padding(8),
                text_input("Absent penalty", &self.state.absent_penalty)
                    .on_input(Message::AbsentPenaltyChanged)
                    .padding(8),
            ]
            .spacing(12),
            row![
                pick_list(
                    ReportFormat::ALL,
                    Some(self.state.report_format),
                    Message::ReportFormatChanged
                ),
                button("Generate Report").on_press(Message::RunAnalysis),
                button("Export").on_press(Message::ExportReport),
            ]
            .spacing(12),
            text(&self.state.status).size(16),
        ]
        .spacing(12)
        .padding(16);

        let (students_list, detail_view): (Element<Message>, Element<Message>) =
            if let Some(report) = &self.state.report {
                let list = report.students.iter().enumerate().fold(
                    Column::new().spacing(4),
                    |col, (index, student)| {
                        let label =
                            format!("{} {} ({})", student.name, student.surname, student.id);
                        let student_button = button(text(label).size(14));
                        col.push(student_button.on_press(Message::SelectStudent(index)))
                    },
                );
                let list = scrollable(container(list).padding(8))
                    .height(Length::Fill)
                    .into();
                let detail = student_detail_view(report, self.state.selected_student);
                (list, detail)
            } else {
                let placeholder = container(text("No report loaded yet."))
                    .center_x()
                    .center_y()
                    .height(Length::Fill)
                    .into();
                (
                    placeholder,
                    container(text("Select a student to view details.")).into(),
                )
            };

        let content = column![
            input_section,
            row![
                container(students_list)
                    .width(Length::FillPortion(1))
                    .height(Length::Fill),
                container(detail_view)
                    .width(Length::FillPortion(2))
                    .height(Length::Fill),
            ]
            .height(Length::Fill)
            .spacing(16)
            .padding(16)
        ]
        .spacing(16);

        container(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

fn student_detail_view(
    report: &AttendanceReport,
    selected_student: Option<usize>,
) -> Element<'_, Message> {
    if let Some(index) = selected_student
        && let Some(student) = report.students.get(index)
    {
        let score = format!("{:.1}/{:.1}", student.score, report.total_points);
        let header_row = row![
            text("Name").width(Length::Fixed(NAME_COLUMN_WIDTH)),
            text("Surname").width(Length::Fixed(SURNAME_COLUMN_WIDTH)),
            text("ID").width(Length::Fixed(ID_COLUMN_WIDTH)),
            text("Normal").width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            text("Late").width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            text("Absent").width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            text("Score").width(Length::Fixed(SCORE_COLUMN_WIDTH)),
        ]
        .spacing(8)
        .align_items(Alignment::Center);
        let table_row = row![
            text(&student.name).width(Length::Fixed(NAME_COLUMN_WIDTH)),
            text(&student.surname).width(Length::Fixed(SURNAME_COLUMN_WIDTH)),
            text(&student.id).width(Length::Fixed(ID_COLUMN_WIDTH)),
            text(student.normal.to_string()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            text(student.late.to_string()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            text(student.absent.to_string()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            text(score).width(Length::Fixed(SCORE_COLUMN_WIDTH)),
        ]
        .spacing(8)
        .align_items(Alignment::Center);
        let table = column![header_row, table_row]
            .spacing(8)
            .align_items(Alignment::Center);
        let table_scrollable = scrollable(container(table).width(Length::Shrink)).direction(
            scrollable::Direction::Horizontal(scrollable::Properties::new()),
        );

        let pie = Canvas::new(PieChart::new(student))
            .width(Length::Fill)
            .height(Length::Fixed(240.0));

        return column![
            text("Selected Student").size(20),
            table_scrollable,
            Space::with_height(Length::Fixed(0.0)),
            text("Attendance Distribution").size(20),
            pie,
        ]
        .spacing(12)
        .padding(16)
        .into();
    }

    container(text("Select a student to view details."))
        .center_x()
        .center_y()
        .into()
}

async fn pick_directory() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_folder()
}

async fn pick_file() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_file()
}

async fn load_attendance(
    directory: PathBuf,
    config: state::AttendanceConfig,
) -> Result<AttendanceReport, String> {
    state::load_attendance(directory, config)
}

async fn save_report(report: AttendanceReport, format: ReportFormat) -> Result<PathBuf, String> {
    state::save_report(report, format)
}

struct PieChart {
    normal: u32,
    late: u32,
    absent: u32,
}

impl PieChart {
    fn new(student: &StudentRecord) -> Self {
        Self {
            normal: student.normal,
            late: student.late,
            absent: student.absent,
        }
    }
}

impl canvas::Program<Message> for PieChart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let total = (self.normal + self.late + self.absent) as f32;
        if total > 0.0 {
            let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
            let radius = bounds.width.min(bounds.height) * 0.35;
            let mut start_angle = 0.0;
            for (value, color) in [
                (self.normal, Color::from_rgb(0.2, 0.7, 0.3)),
                (self.late, Color::from_rgb(0.95, 0.7, 0.2)),
                (self.absent, Color::from_rgb(0.9, 0.3, 0.3)),
            ] {
                let sweep = (value as f32 / total) * std::f32::consts::TAU;
                if sweep > 0.0 {
                    let arc = canvas::path::Arc {
                        center,
                        radius,
                        start_angle: Radians(start_angle),
                        end_angle: Radians(start_angle + sweep),
                    };
                    let path = canvas::Path::new(|builder| {
                        builder.move_to(center);
                        builder.line_to(Point::new(
                            center.x + radius * start_angle.cos(),
                            center.y + radius * start_angle.sin(),
                        ));
                        builder.arc(arc);
                        builder.line_to(center);
                        builder.close();
                    });
                    frame.fill(&path, color);
                }
                start_angle += sweep;
            }
        } else {
            let text = canvas::Text {
                content: "No attendance data".to_string(),
                position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
                color: Color::from_rgb(0.4, 0.4, 0.4),
                size: Pixels(20.0),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                ..canvas::Text::default()
            };
            frame.fill_text(text);
        }
        vec![frame.into_geometry()]
    }
}
