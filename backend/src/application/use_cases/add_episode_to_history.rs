use std::sync::Arc;

use crate::{domain::entities::UserHistory, infrastructure::adapters::SqlUserHistoryRepository};

pub struct AddEpisodeToHistoryUseCase {
    user_history_repository: Arc<SqlUserHistoryRepository>,
}

impl AddEpisodeToHistoryUseCase {
    pub fn new(user_history_repository: Arc<SqlUserHistoryRepository>) -> Self {
        Self {
            user_history_repository,
        }
    }

    pub async fn execute(
        &self,
        anime_slug: &str,
        episode_slug: &str,
        episode_num: i32,
        user_uuid: &str,
    ) -> Result<(), sqlx::Error> {
        let user_history = UserHistory::new(anime_slug, episode_slug, episode_num, user_uuid);
        self.user_history_repository.create(&user_history).await?;
        Ok(())
    }
}
