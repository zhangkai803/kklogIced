use serde::{Deserialize, Serialize};

use crate::core::user::User;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub user: User,
    pub envs: Vec<String>,        // 新增
    pub namespaces: Vec<String>,  // 新增
    pub deployments: Vec<String>, // 新增
    pub pods: Vec<String>,        // 新增
    pub types: Vec<String>,
}
