use std::sync::Arc;

use crate::core::{config::Config, node::Node};
use iced::Theme;
use serde_yaml::Error;
use tokio_tungstenite::tungstenite::protocol;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    AddSource,
    YamlLoaded(Result<Config, Arc<Error>>),
    SourceSelected(Node),
    WssRead(Option<protocol::Message>),
}
