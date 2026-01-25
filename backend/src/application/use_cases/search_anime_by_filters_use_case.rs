use std::sync::Arc;

use crate::{
    domain::{
        entities::Anime,
        value_objects::{Error, Filters, Pagination},
    },
    infrastructure::adapters::AnimeFlvClient,
};

pub struct SearchAnimeByFiltersUseCase {
    pub anime_client: Arc<AnimeFlvClient>,
}

impl SearchAnimeByFiltersUseCase {
    pub async fn execute(
        &self,
        filters: Option<Filters>,
        page: u32,
        order: &str,
    ) -> Result<Pagination<Anime>, Error> {
        let pagination = self
            .anime_client
            .search_anime_by_filters(filters, page, order)
            .await
            .map_err(|_| Error::not_found("Animes not founds"))?;

        Ok(pagination)
    }
}
