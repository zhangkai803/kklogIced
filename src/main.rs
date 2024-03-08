use iced::Application;
use iced::Settings;

mod components;
mod core;
mod layout;
mod message;

use layout::Layout;

pub fn main() -> iced::Result {
    Layout::run(Settings::default())
}
