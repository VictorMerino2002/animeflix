use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    application::use_cases::{
        GetAnimeBySlugUseCase, GetEpisodeBySlugAndAddToHistoryUseCase, SearchAnimeUseCase,
    },
    infrastructure::{
        bootstrap::DepContianer,
        http::{
            ApiResponse,
            extractors::{AuthToken, AuthUser},
        },
    },
};

#[derive(Debug, Deserialize)]
pub struct SearchQueryParams {
    page: u32,
    query: String,
}

pub async fn search_anime(
    AuthToken(_): AuthToken,
    State(container): State<Arc<DepContianer>>,
    Query(params): Query<SearchQueryParams>,
) -> impl IntoResponse {
    let use_case = SearchAnimeUseCase {
        anime_client: container.anime_client.clone(),
    };

    let pagination = use_case.execute(&params.query, params.page).await;
    ApiResponse::default(pagination)
}

pub async fn get_anime_by_slug(
    AuthUser(user): AuthUser,
    State(container): State<Arc<DepContianer>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let use_case = GetAnimeBySlugUseCase {
        anime_client: container.anime_client.clone(),
        user_history_repository: container.user_history_repository.clone(),
    };

    let anime = use_case.execute(&slug, &user.uuid).await;
    ApiResponse::default(anime)
}

pub async fn get_episode_by_slug(
    AuthUser(user): AuthUser,
    State(container): State<Arc<DepContianer>>,
    Path((anime_slug, episode_slug)): Path<(String, String)>,
) -> impl IntoResponse {
    let use_case = GetEpisodeBySlugAndAddToHistoryUseCase {
        anime_client: container.anime_client.clone(),
        user_history_repository: container.user_history_repository.clone(),
    };

    let episode = use_case
        .execute(&episode_slug, &anime_slug, &user.uuid)
        .await;
    ApiResponse::default(episode)
}
