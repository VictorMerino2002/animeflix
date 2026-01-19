use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    application::use_cases::GetAnimeUseCase,
    infrastructure::{di::DepContianer, http::route_utils::handle_response},
};

pub async fn get_anime_by_slug(
    State(container): State<Arc<DepContianer>>,
    Path((slug, user_uuid)): Path<(String, String)>,
) -> impl IntoResponse {
    let use_case = GetAnimeUseCase::new(
        container.anime_client.clone(),
        container.user_history_repository.clone(),
    );
    let response = use_case.execute(&slug, &user_uuid).await;
    handle_response(response, StatusCode::NOT_FOUND)
}

pub async fn get_episode_by_slug(
    State(container): State<Arc<DepContianer>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let response = container.anime_client.get_episode_by_slug(&slug).await;
    handle_response(response, StatusCode::NOT_FOUND)
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    page: u32,
    query: String,
}

pub async fn search_anime(
    State(container): State<Arc<DepContianer>>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    let response = container
        .anime_client
        .search_anime(&params.query, params.page)
        .await;
    handle_response(response, StatusCode::NOT_FOUND)
}
