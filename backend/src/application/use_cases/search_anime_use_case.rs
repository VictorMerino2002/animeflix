use std::sync::Arc;

use crate::{
    domain::{
        entities::Anime,
        value_objects::{Error, Pagination},
    },
    infrastructure::adapters::AnimeFlvClient,
};

pub struct SearchAnimeUseCase {
    pub anime_client: Arc<AnimeFlvClient>,
}

impl SearchAnimeUseCase {
    pub async fn execute(&self, query: &str, page: u32) -> Result<Pagination<Anime>, Error> {
        let pagination = self
            .anime_client
            .search_anime(query, page)
            .await
            .map_err(|_| Error::not_found("Error trying to find the anime"))?;

        Ok(pagination)
    }
}
