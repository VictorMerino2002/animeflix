use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserHistory {
    pub uuid: String,
    pub anime_slug: String,
    pub episode_slug: String,
    pub episode_num: u32,
    pub user_uuid: String,
}
