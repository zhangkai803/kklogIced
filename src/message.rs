use crate::core::config::Config;
use iced::Theme;
use serde_yaml::Error;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelected(Theme),
    AddSource,
    YamlLoaded(Result<Config, Arc<Error>>),
    WssRead(String),
    CloseConnection(u32),      // 新增：关闭指定ID的连接
    EnvSelected(String),       // 新增
    NamespaceSelected(String), // 新增
    DeploymentSelected(String),   // 新增
    PodSelected(String),       // 新增
    TypeSelected(String),
}
