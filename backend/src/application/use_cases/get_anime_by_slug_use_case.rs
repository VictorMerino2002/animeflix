use std::{collections::HashSet, sync::Arc};

use crate::{
    domain::{entities::Anime, value_objects::Error},
    infrastructure::adapters::{AnimeFlvClient, SqlUserHistoryRepository},
};

pub struct GetAnimeBySlugUseCase {
    pub anime_client: Arc<AnimeFlvClient>,
    pub user_history_repository: Arc<SqlUserHistoryRepository>,
}

impl GetAnimeBySlugUseCase {
    pub async fn execute(&self, anime_slug: &str, user_uuid: &str) -> Result<Anime, Error> {
        let error_msg = format!("Error trying to get anime whit slug: `{}`", anime_slug);
        let mut anime = self
            .anime_client
            .get_anime_by_slug(anime_slug)
            .await
            .map_err(|_| Error::not_found(&error_msg))?;

        let episode_history = self
            .user_history_repository
            .get_user_episode_history_by_anime(anime_slug, user_uuid)
            .await
            .map_err(|_| Error::internal_server(&error_msg))?;

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
