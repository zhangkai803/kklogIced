use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct User {
    // pub name: String,
    pub token: String,
}
