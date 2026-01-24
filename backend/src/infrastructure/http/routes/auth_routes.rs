use std::sync::Arc;

use crate::{
    application::use_cases::{CreateUserUseCase, LoginUseCase},
    infrastructure::{
        bootstrap::DepContianer,
        http::{ApiResponse, requests::AuthRequest},
    },
};
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};

pub async fn register(
    State(container): State<Arc<DepContianer>>,
    Json(payload): Json<AuthRequest>,
) -> impl IntoResponse {
    let use_case = CreateUserUseCase {
        user_repository: container.user_repository.clone(),
        password_encryptor: container.password_encryptor.clone(),
    };

    let user = use_case.execute(&payload.username, &payload.password).await;
    ApiResponse::default(user).with_code(StatusCode::CREATED)
}

pub async fn login(
    State(container): State<Arc<DepContianer>>,
    Json(payload): Json<AuthRequest>,
) -> impl IntoResponse {
    let use_case = LoginUseCase {
        user_repository: container.user_repository.clone(),
        password_encryptor: container.password_encryptor.clone(),
        jwt_encoder: container.jwt_encoder.clone(),
    };

    let token = use_case.execute(&payload.username, &payload.password).await;
    ApiResponse::default(token)
}
