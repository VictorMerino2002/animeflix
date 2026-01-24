use std::sync::Arc;

use crate::{
    domain::{entities::Anime, value_objects::Error},
    infrastructure::adapters::AnimeFlvClient,
};

pub struct GetOnAirAnimesUseCase {
    pub anime_client: Arc<AnimeFlvClient>,
}

impl GetOnAirAnimesUseCase {
    pub async fn execute(&self) -> Result<Vec<Anime>, Error> {
        self.anime_client
            .get_on_air_animes()
            .await
            .map_err(|_| Error::not_found("Error trying to get animes"))
    }
}
