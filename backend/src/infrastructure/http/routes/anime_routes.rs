use std::sync::Arc;

use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{
    application::use_cases::{
        GetAnimeBySlugUseCase, GetEpisodeBySlugAndAddToHistoryUseCase, GetOnAirAnimesUseCase,
        GetUserAnimeHistoryUseCase, SearchAnimeByFiltersUseCase, SearchAnimeUseCase,
    },
    domain::value_objects::{Error, Filters},
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

pub async fn get_on_air_animes(
    AuthToken(_): AuthToken,
    State(container): State<Arc<DepContianer>>,
) -> impl IntoResponse {
    let use_case = GetOnAirAnimesUseCase {
        anime_client: container.anime_client.clone(),
    };

    let animes = use_case.execute().await;
    ApiResponse::default(animes)
}

pub async fn get_anime_history(
    AuthUser(user): AuthUser,
    State(container): State<Arc<DepContianer>>,
) -> impl IntoResponse {
    let use_case = GetUserAnimeHistoryUseCase {
        user_history_repository: container.user_history_repository.clone(),
        anime_client: container.anime_client.clone(),
    };

    let animes = use_case.execute(&user.uuid).await;
    ApiResponse::default(animes)
}

#[derive(Deserialize)]
pub struct SearchByFiltersQueryParams {
    pub page: u32,
    pub order: String,
}

pub async fn search_anime_by_filters(
    AuthToken(_): AuthToken,
    State(container): State<Arc<DepContianer>>,
    Query(query): Query<SearchByFiltersQueryParams>,
    body: Bytes,
) -> impl IntoResponse {
    let filters: Option<Filters> = if body.is_empty() {
        None
    } else {
        match serde_json::from_slice(&body) {
            Ok(f) => Some(f),
            Err(_) => {
                return ApiResponse::default(Err(Error::bad_request("Invalid Json body")));
            }
        }
    };

    let use_case = SearchAnimeByFiltersUseCase {
        anime_client: container.anime_client.clone(),
    };
    let result = use_case.execute(filters, query.page, &query.order).await;
    ApiResponse::default(result)
}
