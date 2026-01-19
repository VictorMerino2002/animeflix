use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    application::use_cases::AddEpisodeToHistoryUseCase,
    infrastructure::{di::DepContianer, http::route_utils::handle_response},
};

#[derive(Debug, Deserialize)]
pub struct AddEpsiodeToHistoryRequest {
    anime_slug: String,
    episode_slug: String,
    episode_num: i32,
    user_uuid: String,
}

pub async fn add_episode_to_history(
    State(container): State<Arc<DepContianer>>,
    Json(payload): Json<AddEpsiodeToHistoryRequest>,
) -> impl IntoResponse {
    let use_case = AddEpisodeToHistoryUseCase::new(container.user_history_repository.clone());
    let response = use_case
        .execute(
            &payload.anime_slug,
            &payload.episode_slug,
            payload.episode_num,
            &payload.user_uuid,
        )
        .await;
    handle_response(response, StatusCode::BAD_REQUEST)
}
