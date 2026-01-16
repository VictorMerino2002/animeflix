use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{domain::value_objects::ApiResponse, infrastructure::adapters::AnimeFlvClient};

fn handle_response<T, E>(response: Result<T, E>, error_status_code: StatusCode) -> Response
where
    T: Serialize,
    E: std::fmt::Display,
{
    match response {
        Ok(ok) => (StatusCode::OK, Json(ApiResponse::success(ok))).into_response(),
        Err(err) => (error_status_code, Json(ApiResponse::error(err.to_string()))).into_response(),
    }
}

pub async fn get_anime_by_slug(
    State(client): State<Arc<AnimeFlvClient>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let result = client.get_anime_by_slug(&slug).await;
    handle_response(result, StatusCode::NOT_FOUND)
}

pub async fn get_episode_by_slug(
    State(client): State<Arc<AnimeFlvClient>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let response = client.get_episode_by_slug(&slug).await;
    handle_response(response, StatusCode::NOT_FOUND)
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    page: u32,
    query: String,
}

pub async fn search_anime(
    State(client): State<Arc<AnimeFlvClient>>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    let response = client.search_anime(&params.query, params.page).await;
    handle_response(response, StatusCode::NOT_FOUND)
}
