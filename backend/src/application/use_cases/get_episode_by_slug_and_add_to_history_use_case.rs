use std::sync::Arc;

use crate::{
    domain::{
        entities::{Episode, UserHistory},
        value_objects::Error,
    },
    infrastructure::adapters::{AnimeFlvClient, SqlUserHistoryRepository},
};

pub struct GetEpisodeBySlugAndAddToHistoryUseCase {
    pub anime_client: Arc<AnimeFlvClient>,
    pub user_history_repository: Arc<SqlUserHistoryRepository>,
}

impl GetEpisodeBySlugAndAddToHistoryUseCase {
    pub async fn execute(
        &self,
        episode_slug: &str,
        anime_slug: &str,
        user_uuid: &str,
    ) -> Result<Episode, Error> {
        let episode = self
            .anime_client
            .get_episode_by_slug(episode_slug)
            .await
            .map_err(|_| Error::not_found("Episode not found"))?;

        let history_record = UserHistory::new(anime_slug, episode_slug, episode.number, user_uuid);

        self.user_history_repository
            .create(&history_record)
            .await
            .map_err(|_| Error::internal_server("Error getting episode"))?;

        Ok(episode)
    }
}
