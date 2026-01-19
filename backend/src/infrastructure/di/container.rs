use std::sync::Arc;

use crate::infrastructure::adapters::{
    AnimeFlvClient, SqlUserHistoryRepository, SqlUserRepository,
};

pub struct DepContianer {
    pub anime_client: Arc<AnimeFlvClient>,
    pub user_repository: Arc<SqlUserRepository>,
    pub user_history_repository: Arc<SqlUserHistoryRepository>,
}
