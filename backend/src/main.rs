use std::sync::Arc;

use axum::{Router, routing::get};
use reqwest::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::infrastructure::{
    adapters::AnimeFlvClient,
    http::routes::anime_routes::{get_anime_by_slug, get_episode_by_slug, search_anime},
};

mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let client = Arc::new(AnimeFlvClient::new());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/anime/episode/{slug}", get(get_episode_by_slug))
        .route("/anime/{slug}", get(get_anime_by_slug))
        .route("/anime/search", get(search_anime))
        .with_state(client.clone())
        .layer(cors);

    let addrs = String::from("0.0.0.0:8080");
    let listener = tokio::net::TcpListener::bind(&addrs).await.unwrap();
    println!("Listening on: http://{}", &addrs);
    axum::serve(listener, app).await.unwrap();
}
