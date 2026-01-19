use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: Option<String>,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        let uuid = Uuid::new_v4();

        Self {
            uuid: uuid.to_string(),
            username,
            password: Some(password),
        }
    }
}
