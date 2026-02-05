mod messages;
mod state;
mod style;

use std::path::PathBuf;

use iced::alignment::{Horizontal, Vertical};
use iced::mouse;
use iced::widget::canvas::{self, Canvas};
use iced::widget::{
    Column, Space, button, column, container, pick_list, row, scrollable, text, text_input,
};
use iced::{
    Alignment, Application, Color, Command, Element, Font, Length, Pixels, Point, Radians, Rectangle,
    Renderer, Settings, Size, Theme, executor, theme,
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
        antialiasing: true,
        window: iced::window::Settings {
            min_size: Some(Size::new(900.0, 900.0)),
            ..iced::window::Settings::default()
        },
        default_font: Font::MONOSPACE,
        ..Settings::default()
    })
}

fn labeled_input<'a>(
    label: &'a str,
    value: &'a str,
    on_change: fn(String) -> Message,
) -> Element<'a, Message> {
    container(
        column![
            text(label).size(14).style(style::YELLOW),
            text_input("", value)
                .on_input(on_change)
                .style(theme::TextInput::Custom(Box::new(style::BorderlessInput)))
                .padding(0)
                .size(14)
                .width(Length::Fill)
        ]
        .spacing(2)
        .padding(6),
    )
    .style(theme::Container::Custom(Box::new(style::InputGroup)))
    .width(Length::Fill)
    .into()
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
            text("CONFIGURATION").size(14).style(style::BASE1),
            Space::with_height(Length::Fixed(10.0)),
            row![
                column![
                    text("Attendance Source").size(12).style(style::BASE00),
                    row![
                        text_input("Directory or file path...", &self.state.directory)
                            .on_input(Message::DirectoryChanged)
                            .style(theme::TextInput::Custom(Box::new(style::TextInput)))
                            .padding(8)
                            .width(Length::Fill),
                        button(text("Folder").size(14))
                            .on_press(Message::PickDirectory)
                            .style(theme::Button::Custom(Box::new(style::Button)))
                            .padding(8),
                        button(text("File").size(14))
                            .on_press(Message::PickFile)
                            .style(theme::Button::Custom(Box::new(style::Button)))
                            .padding(8)
                    ]
                    .spacing(8)
                ]
                .width(Length::FillPortion(2))
                .spacing(4),
                column![
                    text("Class Time").size(12).style(style::BASE00),
                    row![
                        labeled_input("Start", &self.state.class_start, Message::ClassStartChanged),
                        labeled_input("End", &self.state.class_end, Message::ClassEndChanged),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center)
                ]
                .width(Length::FillPortion(1))
                .spacing(4),
            ]
            .spacing(20),
            row![
                column![
                    text("Thresholds (Min)").size(12).style(style::BASE00),
                    row![
                        labeled_input("Late", &self.state.late_minutes, Message::LateMinutesChanged),
                        labeled_input("Absent", &self.state.absent_minutes, Message::AbsentMinutesChanged),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center)
                ]
                .width(Length::Fill)
                .spacing(4),
                column![
                    text("Grading (Points)").size(12).style(style::BASE00),
                    row![
                        labeled_input("Total", &self.state.total_points, Message::TotalPointsChanged),
                        labeled_input("Late Pen.", &self.state.late_penalty, Message::LatePenaltyChanged),
                        labeled_input("Absent Pen.", &self.state.absent_penalty, Message::AbsentPenaltyChanged),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center)
                ]
                .width(Length::Fill)
                .spacing(4),
                column![
                    text("Actions").size(12).style(style::BASE00),
                    row![
                        pick_list(
                            ReportFormat::ALL,
                            Some(self.state.report_format),
                            Message::ReportFormatChanged
                        )
                        .style(theme::PickList::Custom(
                            std::rc::Rc::new(style::PickList),
                            std::rc::Rc::new(style::Menu)
                        )),
                        button(text("ANALYZE").size(14))
                            .on_press(Message::RunAnalysis)
                            .style(theme::Button::Custom(Box::new(style::PrimaryButton)))
                            .padding(8),
                        button(text("EXPORT").size(14))
                            .on_press(Message::ExportReport)
                            .style(theme::Button::Custom(Box::new(style::Button)))
                            .padding(8),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center)
                ]
                .spacing(4)
            ]
            .spacing(20),
        ]
        .spacing(12)
        .padding(16);

        let input_container = container(input_section)
            .style(theme::Container::Custom(Box::new(style::Panel)))
            .width(Length::Fill);

        let (students_list, detail_view): (Element<Message>, Element<Message>) =
            if let Some(report) = &self.state.report {
                let list = report.students.iter().enumerate().fold(
                    Column::new().spacing(4),
                    |col, (index, student)| {
                        let label = format!(
                            "{:<20} {:<20} ({})",
                            student.surname, student.name, student.id
                        );
                        let student_button = button(text(label).size(14))
                            .style(if self.state.selected_student == Some(index) {
                                theme::Button::Custom(Box::new(style::PrimaryButton))
                            } else {
                                theme::Button::Custom(Box::new(style::Button))
                            })
                            .width(Length::Fill);
                        col.push(student_button.on_press(Message::SelectStudent(index)))
                    },
                );
                let list = scrollable(container(list).padding(8))
                    .height(Length::Fill)
                    .style(theme::Scrollable::Custom(Box::new(style::Scrollable)))
                    .into();
                let detail = student_detail_view(report, self.state.selected_student);
                (list, detail)
            } else {
                let placeholder = container(text("No report loaded yet.").style(style::BASE01))
                    .center_x()
                    .center_y()
                    .height(Length::Fill)
                    .into();
                (
                    placeholder,
                    container(
                        text("Select a student to view details.")
                            .style(style::BASE01)
                            .size(18),
                    )
                    .center_x()
                    .center_y()
                    .into(),
                )
            };

        let list_container = container(
            column![
                text("STUDENTS").size(14).style(style::BASE1),
                Space::with_height(Length::Fixed(10.0)),
                students_list
            ]
            .padding(16),
        )
        .style(theme::Container::Custom(Box::new(style::Panel)))
        .width(Length::FillPortion(1))
        .height(Length::Fill);

        let detail_container = container(detail_view)
            .style(theme::Container::Custom(Box::new(style::Panel)))
            .width(Length::FillPortion(2))
            .height(Length::Fill);

        let status_bar = container(
            row![
                text("STATUS: ").style(style::BASE1).size(14),
                text(&self.state.status).style(style::BASE0).size(14)
            ]
            .padding(4),
        )
        .style(theme::Container::Custom(Box::new(style::BorderedPanel)))
        .width(Length::Fill);

        let content = column![
            input_container,
            row![list_container, detail_container]
                .height(Length::Fill)
                .spacing(16),
            status_bar
        ]
        .spacing(16)
        .padding(16);

        container(content)
            .height(Length::Fill)
            .width(Length::Fill)
            .style(theme::Container::Custom(Box::new(style::MainBg)))
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

        // Helper for table cells
        let cell = |content: String| container(text(content).size(14)).padding(4);

        let header_row = row![
            cell("Name".into()).width(Length::Fixed(NAME_COLUMN_WIDTH)),
            cell("Surname".into()).width(Length::Fixed(SURNAME_COLUMN_WIDTH)),
            cell("ID".into()).width(Length::Fixed(ID_COLUMN_WIDTH)),
            cell("Normal".into()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            cell("Late".into()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            cell("Absent".into()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            cell("Score".into()).width(Length::Fixed(SCORE_COLUMN_WIDTH)),
        ]
        .spacing(8)
        .align_items(Alignment::Center);

        let table_row = row![
            cell(student.name.clone()).width(Length::Fixed(NAME_COLUMN_WIDTH)),
            cell(student.surname.clone()).width(Length::Fixed(SURNAME_COLUMN_WIDTH)),
            cell(student.id.clone()).width(Length::Fixed(ID_COLUMN_WIDTH)),
            cell(student.normal.to_string()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            cell(student.late.to_string()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            cell(student.absent.to_string()).width(Length::Fixed(COUNT_COLUMN_WIDTH)),
            cell(score).width(Length::Fixed(SCORE_COLUMN_WIDTH)),
        ]
        .spacing(8)
        .align_items(Alignment::Center);

        let table = column![
            container(header_row).style(theme::Container::Custom(Box::new(style::BorderedPanel))).padding(4),
            container(table_row).padding(4)
        ]
        .spacing(8);

        let table_scrollable = scrollable(container(table).width(Length::Shrink))
            .direction(scrollable::Direction::Horizontal(scrollable::Properties::new()))
            .style(theme::Scrollable::Custom(Box::new(style::Scrollable)));

        let pie = Canvas::new(PieChart::new(student))
            .width(Length::Fill)
            .height(Length::Fixed(240.0));

        let legend_item = |label: &str, color: Color| {
            row![
                container(Space::new(Length::Fixed(16.0), Length::Fixed(16.0)))
                    .style(theme::Container::Custom(Box::new(style::ColoredBox(color)))),
                text(label).size(12).style(style::BASE0),
            ]
            .spacing(4)
            .align_items(Alignment::Center)
        };

        let legend = row![
            legend_item("Normal", style::CHART_GREEN),
            legend_item("Late", style::CHART_YELLOW),
            legend_item("Absent", style::CHART_RED),
        ]
        .spacing(16)
        .align_items(Alignment::Center);

        return column![
            text("SELECTED STUDENT").size(14).style(style::BASE1),
            Space::with_height(Length::Fixed(10.0)),
            table_scrollable,
            Space::with_height(Length::Fixed(20.0)),
            text("ATTENDANCE DISTRIBUTION").size(14).style(style::BASE1),
            pie,
            container(legend).width(Length::Fill).center_x(),
        ]
        .spacing(12)
        .padding(16)
        .into();
    }

    container(
        text("Select a student to view details.")
            .style(style::BASE01)
            .size(18),
    )
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
                (self.normal, style::CHART_GREEN),
                (self.late, style::CHART_YELLOW),
                (self.absent, style::CHART_RED),
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
                    frame.stroke(
                        &path,
                        canvas::Stroke::default()
                            .with_color(style::BASE03)
                            .with_width(2.0),
                    );
                }
                start_angle += sweep;
            }
        } else {
            let text = canvas::Text {
                content: "No attendance data".to_string(),
                position: Point::new(bounds.width / 2.0, bounds.height / 2.0),
                color: style::BASE01,
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
