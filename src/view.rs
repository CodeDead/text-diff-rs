use iced::alignment::{self};
use iced::{
    button, text_input, Alignment, Button, Column, Container, Element, Length, Row, Sandbox, Text,
    TextInput,
};
use native_dialog::FileDialog;

#[derive(Debug, Clone)]
pub enum Message {
    FirstFileInputChanged(String),
    SecondFileInputChanged(String),
    SelectFirstFilePressed,
    SelectSecondFilePressed,
    ComparePressed,
}

#[derive(Default)]
pub struct ApplicationContext {
    pub first_file: String,
    pub second_file: String,
    pub first_file_input: text_input::State,
    pub second_file_input: text_input::State,
    pub btn_select_first_file: button::State,
    pub btn_select_second_file: button::State,
    pub btn_compare: button::State,
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
                println!("Hello from btnCompare");
            }
        };
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let title = Text::new("text-diff")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(alignment::Horizontal::Center);

        let first_file_input = TextInput::new(
            &mut self.first_file_input,
            "/path/to/first/file.txt",
            &self.first_file,
            Message::FirstFileInputChanged,
        )
        .padding(10)
        .size(20);

        let btn_select_first_file = Button::new(
            &mut self.btn_select_first_file,
            Text::new("...").horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding(10)
        .min_width(60)
        .on_press(Message::SelectFirstFilePressed);

        let second_file_input = TextInput::new(
            &mut self.second_file_input,
            "/path/to/second/file.txt",
            &self.second_file,
            Message::SecondFileInputChanged,
        )
        .padding(10)
        .size(20);

        let btn_select_second_file = Button::new(
            &mut self.btn_select_second_file,
            Text::new("...").horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding(10)
        .min_width(60)
        .on_press(Message::SelectSecondFilePressed);

        let btn_compare = Button::new(&mut self.btn_compare, Text::new("Compare"))
            .padding(10)
            .on_press(Message::ComparePressed);

        let content = Column::new()
            .spacing(15)
            .padding(20)
            .max_width(800)
            .push(title)
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
            .into()
    }
}
