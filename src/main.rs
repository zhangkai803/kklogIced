use iced::application;
use iced::executor;
use iced::keyboard;
use iced::mouse;
use iced::theme;
use iced::widget::{
    button, canvas, checkbox, column, container, horizontal_space, pick_list, row, scrollable, text,
};
use iced::{
    color, Alignment, Application, Command, Element, Font, Length, Point, Rectangle, Renderer,
    Settings, Subscription, Theme,
};

mod application;
mod components;
mod core;
mod message;

use application::Layout;
use message::Message;

pub fn main() -> iced::Result {
    Layout::run(Settings::default())
}
