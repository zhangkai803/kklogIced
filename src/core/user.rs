use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct User {
    pub name: String,
    pub token: String,
}
