use iced::advanced::text::Shaping;
use iced::widget::scrollable::Direction;
use iced::widget::scrollable::Properties;
use iced::widget::Column;
use iced::widget::{container, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;

#[derive(Debug, Default)]
pub struct Stream {
    pub title: String,
    pub url: String,
    pub buf: Vec<String>,
}

impl Stream {
    pub fn new(title: String, url: String) -> Self {
        Self {
            title,
            url,
            buf: Vec::<String>::default(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(
            scrollable(
                Column::with_children(
                    self.buf
                        .iter()
                        .map(|s| text(s).shaping(Shaping::Advanced).into()),
                )
                .spacing(40)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            )
            .direction(Direction::Vertical(
                Properties::default().alignment(iced::widget::scrollable::Alignment::End),
            ))
            .height(Length::Fill),
        )
        .padding(10)
        .into()
    }
}
