use chrono::{NaiveDateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserHistory {
    pub uuid: String,
    pub anime_slug: String,
    pub episode_slug: String,
    pub episode_num: i32,
    pub user_uuid: String,
    pub created_at: NaiveDateTime,
}

impl UserHistory {
    pub fn new(anime_slug: &str, episode_slug: &str, episode_num: i32, user_uuid: &str) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let created_at = Utc::now().naive_utc();

        Self {
            uuid,
            anime_slug: anime_slug.to_string(),
            episode_slug: episode_slug.to_string(),
            episode_num,
            user_uuid: user_uuid.to_string(),
            created_at,
        }
    }
}
