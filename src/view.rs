use crate::filereader::FileReader;
use crate::style;
use iced::alignment;
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
    pub differences: Vec<String>,
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
                            .set_text(&format!("Error while reading file!\n{}", e))
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
                            .set_text(&format!("Error while reading file!\n{}", e))
                            .show_alert()
                            .unwrap();
                        return;
                    }
                };

                let mut diff = vec![];
                for f in &lines_first_file {
                    let mut included = false;
                    for d in &lines_second_file {
                        if f.eq(d) {
                            included = true;
                        }
                    }

                    if !included {
                        diff.push(String::from(f));
                    }
                }

                for d in &diff {
                    println!("{}", &d);
                }

                self.differences = diff;
            }
            Message::ThemeChanged(d) => self.theme = d,
        };
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = Text::new("text-diff")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(alignment::Horizontal::Center);

        let choose_theme = style::Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose a theme:")),
            |column, theme| {
                column.push(
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

        let btn_compare = Button::new(&mut self.btn_compare, Text::new("Compare"))
            .padding(10)
            .on_press(Message::ComparePressed)
            .style(self.theme);

        let content = Column::new()
            .spacing(15)
            .padding(20)
            .max_width(800)
            .push(title)
            .push(choose_theme)
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
            .push(
                Row::new().spacing(10).push(
                    Column::new()
                        .width(Length::Fill)
                        .align_items(Alignment::End)
                        .spacing(20)
                        .push(btn_compare),
                ),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }
}
