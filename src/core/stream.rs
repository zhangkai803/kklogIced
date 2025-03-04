use iced::advanced::text::Shaping;
use iced::widget::scrollable::Direction;
use iced::widget::scrollable::Properties;
use iced::widget::Column;
use iced::widget::{container, scrollable, text};
use iced::{Alignment, Element, Length};

use crate::message::Message;

use super::config::Pod;
use super::node::Node;

#[derive(Debug)]
pub struct Stream {
    pub url: String,
    pub buf: Vec<String>,
    pub connection_id: u32,
    pub namespace: String,
    pub pod: Pod,
}

impl Stream {

    pub fn from_selected(
        env: String,
        namespace: String,
        deployment: String,
        pod: Pod,
        token: String,
    ) -> Self {
        let node = Node {
            env,
            namespace: namespace.clone(),
            deployment,
            pod: pod.name.clone(),
            r#type: pod.r#type.clone(),
        };

        Self {
            url: node.url(&token),
            buf: Vec::<String>::default(),
            connection_id: rand::random::<u32>(),
            namespace,
            pod,
        }
    }

    pub fn view(&self) -> Element<Message> {
        container(
            scrollable(
                Column::with_children(
                    self.buf
                        .iter()
                        .map(|s| text(
                            "[".to_owned() + &self.namespace.clone() + "]" + &self.pod.to_string() + " " + s.split_once(":").map(|(_, second)| second).unwrap()
                        ).shaping(Shaping::Advanced).into()),
                )
                .spacing(5)
                .align_items(Alignment::Start)
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
            namespace: String::default(),
            pod: Pod::default(),
        }
    }
}
