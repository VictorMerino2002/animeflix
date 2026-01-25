use std::sync::Arc;

use crate::{
    domain::{entities::Anime, value_objects::Error},
    infrastructure::adapters::{AnimeFlvClient, HttpError, SqlUserHistoryRepository},
};
use futures::future::join_all;

pub struct GetUserAnimeHistoryUseCase {
    pub user_history_repository: Arc<SqlUserHistoryRepository>,
    pub anime_client: Arc<AnimeFlvClient>,
}

impl GetUserAnimeHistoryUseCase {
    pub async fn execute(&self, user_uuid: &str) -> Result<Vec<Anime>, Error> {
        let history_records = self
            .user_history_repository
            .get_user_anime_history(user_uuid)
            .await
            .map_err(|_| Error::internal_server("Error getting animes"))?;

        let anime_futures = history_records
            .iter()
            .map(|record| self.get_anime(&record.anime_slug));

        let results = join_all(anime_futures).await;

        let animes: Vec<Anime> = results.into_iter().filter_map(Result::ok).collect();

        Ok(animes)
    }

    async fn get_anime(&self, slug: &str) -> Result<Anime, HttpError> {
        let mut anime = self.anime_client.get_anime_by_slug(slug).await?;
        anime.slug = Some(slug.to_string());
        Ok(anime)
    }
}
