use iced::window;
use iced::{Sandbox, Settings};

mod view;
mod style;
mod filereader;
mod vector_comparer;

pub fn main() -> iced::Result {
    view::ApplicationContext::run(Settings {
        id: Some(String::from("text-diff")),
        window: window::Settings {
            size: (800, 600),
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Default::default()
    })
}
