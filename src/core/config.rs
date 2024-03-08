use serde::Deserialize;

use crate::core::node::Node;
use crate::core::user::User;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    pub user: User,
    pub envs: Vec<Node>,
}
