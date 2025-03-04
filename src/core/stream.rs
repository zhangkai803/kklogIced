use iced::advanced::text::Shaping;
use iced::widget::scrollable::Direction;
use iced::widget::scrollable::Properties;
use iced::widget::Column;
use iced::widget::{container, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;

use super::node::Node;

#[derive(Debug)]
pub struct Stream {
    pub url: String,
    pub buf: Vec<String>,
    pub connection_id: u32,
}

impl Stream {

    pub fn from_selected(
        env: String,
        namespace: String,
        deployment: String,
        r#type: String,
        pod: String,
        token: String,
    ) -> Self {
        let node = Node {
            env,
            namespace,
            deployment,
            pod,
            r#type,
        };

        Self {
            url: node.url(&token),
            buf: Vec::<String>::default(),
            connection_id: rand::random::<u32>(),
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

impl Default for Stream {
    fn default() -> Self {
        Self {
            url: String::default(),
            buf: Vec::default(),
            connection_id: 0,
        }
    }
}
