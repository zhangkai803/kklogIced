use iced::{Application, Settings};

mod components;
mod core;
mod layout;
mod message;
mod style;

pub fn main() -> iced::Result {
    layout::Layout::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
