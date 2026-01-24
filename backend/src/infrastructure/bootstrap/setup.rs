use std::{error::Error, sync::Arc};

use crate::infrastructure::{
    adapters::{
        AnimeFlvClient, ArgonPasswordEncriptor, JwtEncoder, SqlUserHistoryRepository,
        SqlUserRepository,
    },
    bootstrap::DepContianer,
};

pub async fn setup() -> Result<Arc<DepContianer>, Box<dyn Error>> {
    let max_connections = 10;
    let db_url = "postgres://admin:super@0.0.0.0:5432/animeflix";

    let anime_client = AnimeFlvClient::new();
    let user_repository = SqlUserRepository::new(db_url, max_connections).await?;
    let user_history_repository = SqlUserHistoryRepository::new(db_url, max_connections).await?;
    let password_encryptor = ArgonPasswordEncriptor {};
    let jwt_encoder = JwtEncoder::new();

    Ok(Arc::new(DepContianer {
        anime_client: Arc::new(anime_client),
        user_repository: Arc::new(user_repository),
        user_history_repository: Arc::new(user_history_repository),
        password_encryptor: Arc::new(password_encryptor),
        jwt_encoder: Arc::new(jwt_encoder),
    }))
}
