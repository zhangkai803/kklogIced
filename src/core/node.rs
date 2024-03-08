use iced::{widget::text, Element};

use crate::message::Message;

#[derive(Debug, Clone, Default)]
pub struct Node {
    name: String,
    project: String,
    deployment: String,
    r#type: String,
    pod: String,
    namespace: String,
}

impl Node {
    fn view(&self) -> Element<Message> {
        text("text").into()
    }
}
