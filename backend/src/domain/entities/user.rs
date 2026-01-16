use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: Option<String>,
}
