use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Stream {
    pub title: String,
}

impl Stream {
    pub fn new(title: String) -> Self {
        Self {
            title,
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(
            scrollable(
                column![
                    text(&self.title)
                ]
                    .spacing(40)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10)
        .into()
    }
}
