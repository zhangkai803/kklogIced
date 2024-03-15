use crate::core::{config::Config, node::Node};
use iced::Theme;
use serde_yaml::Error;
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    AddSource,
    YamlLoaded(Result<Config, Arc<Error>>),
    SourceSelected(Node),
    WssRead(Option<String>),
    Tick(Instant),
}
