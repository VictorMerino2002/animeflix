use axum::{
    Router,
    routing::{get, post},
};
use reqwest::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::infrastructure::{
    bootstrap::setup,
    http::routes::{
        anime_routes::{
            get_anime_by_slug, get_anime_history, get_episode_by_slug, get_on_air_animes,
            search_anime, search_anime_by_filters,
        },
        auth_routes::{login, register},
    },
};

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let container = setup().await.expect("Error on the setup");

    let app = Router::new()
        // AUTH
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        // ANIME
        .route("/anime/list/on-air", get(get_on_air_animes))
        .route("/anime/search", get(search_anime))
        .route("/anime/search/by-filters", post(search_anime_by_filters))
        .route("/anime/history", get(get_anime_history))
        .route("/anime/{slug}", get(get_anime_by_slug))
        .route(
            "/anime/{anime_slug}/episode/{episode_slug}",
            get(get_episode_by_slug),
        )
        .with_state(container.clone())
        .layer(cors);

    let addrs = String::from("0.0.0.0:8080");
    let listener = tokio::net::TcpListener::bind(&addrs).await.unwrap();
    println!("Listening on: http://{}", &addrs);
    axum::serve(listener, app).await.unwrap();
}
