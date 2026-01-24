use std::sync::Arc;

use crate::infrastructure::adapters::{
    AnimeFlvClient, ArgonPasswordEncriptor, JwtEncoder, SqlUserHistoryRepository, SqlUserRepository,
};

#[derive(Clone)]
pub struct DepContianer {
    pub anime_client: Arc<AnimeFlvClient>,
    pub user_repository: Arc<SqlUserRepository>,
    pub user_history_repository: Arc<SqlUserHistoryRepository>,
    pub password_encryptor: Arc<ArgonPasswordEncriptor>,
    pub jwt_encoder: Arc<JwtEncoder>,
}
