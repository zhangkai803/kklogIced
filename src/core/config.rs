use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::user::User;


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Pod {
    pub name: String,
    pub r#type: String,
}

impl ToString for Pod {
    fn to_string(&self) -> String {
        format!("[{}]", self.name)
    }
}

impl PartialEq for Pod {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub user: User,
    pub envs: Vec<String>,
    pub namespaces: Vec<String>, // 修改
    pub deployments: Vec<String>, // 修改
    pub pods: HashMap<String, Vec<Pod>>,       // 修改
}
