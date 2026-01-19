use std::{collections::HashSet, error::Error, sync::Arc};

use crate::{
    domain::entities::Anime,
    infrastructure::adapters::{AnimeFlvClient, SqlUserHistoryRepository},
};

pub struct GetAnimeUseCase {
    anime_client: Arc<AnimeFlvClient>,
    user_history_repository: Arc<SqlUserHistoryRepository>,
}

impl GetAnimeUseCase {
    pub fn new(
        anime_client: Arc<AnimeFlvClient>,
        user_history_repository: Arc<SqlUserHistoryRepository>,
    ) -> Self {
        Self {
            anime_client,
            user_history_repository,
        }
    }

    pub async fn execute(
        &self,
        anime_slug: &str,
        user_uuid: &str,
    ) -> Result<Anime, Box<dyn Error>> {
        let mut anime = self.anime_client.get_anime_by_slug(anime_slug).await?;

        let episode_history = self
            .user_history_repository
            .get_user_episode_history_by_anime(anime_slug, user_uuid)
            .await?;

        let seen_slugs: HashSet<String> = episode_history
            .iter()
            .map(|h| h.episode_slug.clone())
            .collect();

        for episode in anime.episodes.iter_mut() {
            if let Some(slug) = &episode.slug
                && seen_slugs.contains(slug)
            {
                episode.seen = true;
            }
        }

        let mut last_episode_seen = None;

        if let Some(e) = &episode_history.first() {
            last_episode_seen = Some(e.episode_num);
        }

        anime.last_episode_number_seen = last_episode_seen;

        Ok(anime)
    }
}
