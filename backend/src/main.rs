use std::{error::Error, sync::Arc};

use axum::{
    Router,
    routing::{get, post},
};
use reqwest::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::infrastructure::{
    adapters::{AnimeFlvClient, SqlUserHistoryRepository, SqlUserRepository},
    di::DepContianer,
    http::routes::{
        anime_routes::{get_anime_by_slug, get_episode_by_slug, search_anime},
        history_routes::add_episode_to_history,
    },
};

mod application;
mod domain;
mod infrastructure;

async fn setup() -> Result<Arc<DepContianer>, Box<dyn Error>> {
    let max_connections = 10;
    let db_url = "postgres://admin:super@0.0.0.0:5432/animeflix";

    let anime_client = AnimeFlvClient::new();
    let user_repository = SqlUserRepository::new(db_url, max_connections).await?;
    let user_history_repository = SqlUserHistoryRepository::new(db_url, max_connections).await?;

    Ok(Arc::new(DepContianer {
        anime_client: Arc::new(anime_client),
        user_repository: Arc::new(user_repository),
        user_history_repository: Arc::new(user_history_repository),
    }))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let container = setup().await.expect("Error on the setup");

    let app = Router::new()
        .route("/anime/episode/{slug}", get(get_episode_by_slug))
        .route("/anime/{slug}/{user_uuid}", get(get_anime_by_slug))
        .route("/anime/search", get(search_anime))
        .route("/history/create", post(add_episode_to_history))
        .with_state(container.clone())
        .layer(cors);

    let addrs = String::from("0.0.0.0:8080");
    let listener = tokio::net::TcpListener::bind(&addrs).await.unwrap();
    println!("Listening on: http://{}", &addrs);
    axum::serve(listener, app).await.unwrap();
}
