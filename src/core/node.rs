use crate::message::Message;
use iced::{
    theme,
    widget::{button, text},
    Element,
};
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Node {
    pub source: String,
    pub project: String,
    pub deployment: String,
    pub r#type: String,
    pub name: String,
    pub namespace: String,
}

impl Node {
    pub fn view(&self) -> Element<Message> {
        button(text(self.source.clone()))
            .on_press(Message::SourceSelected(self.to_owned()))
            .style(theme::Button::Text)
            .into()
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
