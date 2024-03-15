use iced::advanced::text::Shaping;
use iced::widget::{column, container, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;

#[derive(Debug, Default, Clone)]
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
        // println!("len of buf: {}", self.buf.len());
        container(
            scrollable(
                column![text(self.buf.join("\n")).shaping(Shaping::Advanced)] // Shaping::Advanced for chinese, thanks to Koranir!
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
