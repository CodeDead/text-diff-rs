use iced::window;
use iced::{Sandbox, Settings};

mod filereader;
mod style;
mod vector_comparer;
mod vector_exporter;
mod view;

pub fn main() -> iced::Result {
    view::ApplicationContext::run(Settings {
        id: Some(String::from("text-diff")),
        window: window::Settings {
            size: (800, 800),
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Default::default()
    })
}
