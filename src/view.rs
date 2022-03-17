use std::ffi::OsStr;
use std::path::Path;

use crate::filereader::FileReader;
use crate::style;
use crate::vector_comparer::{IVectorComparer, VectorComparer};
use crate::vector_exporter::{ExportType, IVectorExporter, VectorExporter};
use iced::{alignment, scrollable, Rule, Scrollable};
use iced::{
    button, text_input, Alignment, Button, Column, Container, Element, Length, Radio, Row, Sandbox,
    Text, TextInput,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(style::Theme),
    FirstFileInputChanged(String),
    SecondFileInputChanged(String),
    SelectFirstFilePressed,
    SelectSecondFilePressed,
    ComparePressed,
    ClearComparePressed,
    ExportPressed,
}

#[derive(Default)]
pub struct ApplicationContext {
    pub theme: style::Theme,
    pub first_file: String,
    pub second_file: String,
    pub first_file_input: text_input::State,
    pub second_file_input: text_input::State,
    pub btn_select_first_file: button::State,
    pub btn_select_second_file: button::State,
    pub btn_compare: button::State,
    pub btn_clean_compare: button::State,
    pub btn_export: button::State,
    pub scrollable: scrollable::State,
    pub differences: Vec<String>,
    pub has_compared: bool,
}

impl Sandbox for ApplicationContext {
    type Message = Message;

    fn new() -> Self {
        ApplicationContext::default()
    }

    fn title(&self) -> String {
        String::from("text-diff")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::FirstFileInputChanged(d) => self.first_file = d,
            Message::SecondFileInputChanged(d) => self.second_file = d,
            Message::SelectFirstFilePressed => {
                let path = FileDialog::new()
                    .add_filter("Text file", &["txt"])
                    .add_filter("All files", &["*"])
                    .show_open_single_file()
                    .unwrap();

                let path = match path {
                    Some(path) => path,
                    None => return,
                };

                self.first_file = path.into_os_string().into_string().unwrap();
            }
            Message::SelectSecondFilePressed => {
                let path = FileDialog::new()
                    .add_filter("Text file", &["txt"])
                    .add_filter("All files", &["*"])
                    .show_open_single_file()
                    .unwrap();

                let path = match path {
                    Some(path) => path,
                    None => return,
                };

                self.second_file = path.into_os_string().into_string().unwrap();
            }
            Message::ComparePressed => {
                if self.first_file.is_empty() || self.second_file.is_empty() {
                    MessageDialog::new()
                        .set_type(MessageType::Warning)
                        .set_title("text-diff")
                        .set_text("Please select two files first!")
                        .show_alert()
                        .unwrap();
                    return;
                }

                let file_reader = FileReader::new();

                let lines_first_file = file_reader.read_lines(&self.first_file);
                let lines_second_file = file_reader.read_lines(&self.second_file);

                let lines_first_file = match lines_first_file {
                    Ok(d) => d,
                    Err(e) => {
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("text-diff")
                            .set_text(&format!(
                                "Error while reading file {}!\n{}",
                                &self.first_file, e
                            ))
                            .show_alert()
                            .unwrap();
                        return;
                    }
                };

                let lines_second_file = match lines_second_file {
                    Ok(d) => d,
                    Err(e) => {
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("text-diff")
                            .set_text(&format!(
                                "Error while reading file {}!\n{}",
                                &self.second_file, e
                            ))
                            .show_alert()
                            .unwrap();
                        return;
                    }
                };

                let vector_comparer: VectorComparer<String> =
                    IVectorComparer::<String>::new(lines_first_file, lines_second_file);

                self.differences = vector_comparer.get_differences();
                self.has_compared = true;
            }
            Message::ThemeChanged(d) => self.theme = d,
            Message::ClearComparePressed => {
                self.first_file = String::new();
                self.second_file = String::new();
                self.has_compared = false;
                self.differences = vec![];
            }
            Message::ExportPressed => {
                let path = FileDialog::new()
                    .add_filter("Text file", &["txt"])
                    .add_filter("Csv file", &["csv"])
                    .add_filter("Json file", &["json"])
                    .show_save_single_file()
                    .unwrap();

                let path = match path {
                    Some(path) => path,
                    None => return,
                };

                let path = path.into_os_string().into_string().unwrap();

                let extension = match Path::new(&path).extension().and_then(OsStr::to_str) {
                    Some(x) => x,
                    None => return,
                };

                let extension = match extension.to_lowercase().as_str() {
                    "txt" => ExportType::Text,
                    "csv" => ExportType::Csv,
                    "json" => ExportType::Json,
                    _ => ExportType::default(),
                };

                let vec_exporter: VectorExporter<String> =
                    IVectorExporter::<String>::new(self.differences.clone(), extension, &path);

                match vec_exporter.export() {
                    Ok(_) => return,
                    Err(e) => match e {
                        crate::vector_exporter::ExportError::IoError(e) => {
                            MessageDialog::new()
                                .set_type(MessageType::Error)
                                .set_title("text-diff")
                                .set_text(&format!("Error while writing to file {}!\n{}", &path, e))
                                .show_alert()
                                .unwrap();
                        }
                        crate::vector_exporter::ExportError::JsonError(e) => {
                            MessageDialog::new()
                                .set_type(MessageType::Error)
                                .set_title("text-diff")
                                .set_text(&format!(
                                    "Error while creating JSON for file {}!\n{}",
                                    &path, e
                                ))
                                .show_alert()
                                .unwrap();
                        }
                    },
                };
            }
        };
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = Text::new("text-diff")
            .width(Length::Fill)
            .size(85)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(alignment::Horizontal::Center);

        let choose_theme = style::Theme::ALL.iter().fold(
            Row::new()
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .spacing(10),
            |row, theme| {
                row.push(
                    Radio::new(
                        *theme,
                        format!("{:?}", theme),
                        Some(self.theme),
                        Message::ThemeChanged,
                    )
                    .style(self.theme),
                )
            },
        );

        let first_file_input = TextInput::new(
            &mut self.first_file_input,
            "/path/to/first/file.txt",
            &self.first_file,
            Message::FirstFileInputChanged,
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let btn_select_first_file = Button::new(
            &mut self.btn_select_first_file,
            Text::new("...").horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding(10)
        .min_width(60)
        .on_press(Message::SelectFirstFilePressed)
        .style(self.theme);

        let second_file_input = TextInput::new(
            &mut self.second_file_input,
            "/path/to/second/file.txt",
            &self.second_file,
            Message::SecondFileInputChanged,
        )
        .padding(10)
        .size(20)
        .style(self.theme);

        let btn_select_second_file = Button::new(
            &mut self.btn_select_second_file,
            Text::new("...").horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding(10)
        .min_width(60)
        .on_press(Message::SelectSecondFilePressed)
        .style(self.theme);

        let btn_compare = Button::new(
            &mut self.btn_compare,
            Text::new("Compare").horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding(10)
        .min_width(100)
        .on_press(Message::ComparePressed)
        .style(self.theme);

        let mut compare_row = Row::new().spacing(10);

        if self.has_compared {
            let btn_clean_compare = Button::new(
                &mut self.btn_clean_compare,
                Text::new("Clear").horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(10)
            .min_width(100)
            .on_press(Message::ClearComparePressed)
            .style(self.theme);

            compare_row = compare_row.push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Start)
                    .spacing(20)
                    .push(btn_clean_compare),
            );
        }

        compare_row = compare_row.push(
            Column::new()
                .width(Length::Fill)
                .align_items(Alignment::End)
                .spacing(20)
                .push(btn_compare),
        );

        let mut content = Column::new()
            .spacing(15)
            .padding(20)
            .max_width(800)
            .push(title)
            .push(Rule::horizontal(20).style(self.theme))
            .push(
                Row::new()
                    .spacing(10)
                    .push(first_file_input)
                    .push(btn_select_first_file),
            )
            .push(
                Row::new()
                    .spacing(10)
                    .push(second_file_input)
                    .push(btn_select_second_file),
            )
            .push(compare_row);

        if self.has_compared {
            let mut diff_text = Text::new("Differences:");
            if self.differences.is_empty() {
                diff_text = Text::new("No differences detected!")
            }

            let diff_column = self.differences.iter().fold(
                Column::new().spacing(10),
                |column, theme| column.push(Text::new(format!("- {}", theme))),
            );

            let scroll_container = Column::new().width(Length::Fill).push(diff_column);
            let scroll = Scrollable::new(&mut self.scrollable)
                .push(Container::new(scroll_container)
                .width(Length::Fill))
                .max_height(150)
                .style(self.theme);

            content = content
                .push(Rule::horizontal(20).style(self.theme))
                .push(diff_text.size(30))
                .push(scroll);

            if !self.differences.is_empty() {
                let btn_export = Button::new(
                    &mut self.btn_export,
                    Text::new("Export").horizontal_alignment(alignment::Horizontal::Center),
                )
                .padding(10)
                .min_width(100)
                .on_press(Message::ExportPressed)
                .style(self.theme);

                content = content.push(
                    Column::new()
                        .width(Length::Fill)
                        .align_items(Alignment::End)
                        .spacing(20)
                        .push(btn_export),
                );
            }
        }

        content = content
            .push(Rule::horizontal(20).style(self.theme))
            .push(choose_theme);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
