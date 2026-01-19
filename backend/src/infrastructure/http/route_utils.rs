use axum::response::IntoResponse;
use axum::{Json, response::Response};
use reqwest::StatusCode;
use serde::Serialize;

use crate::domain::value_objects::ApiResponse;

pub fn handle_response<T, E>(response: Result<T, E>, error_status_code: StatusCode) -> Response
where
    T: Serialize,
    E: std::fmt::Display,
{
    match response {
        Ok(ok) => (StatusCode::OK, Json(ApiResponse::success(ok))).into_response(),
        Err(err) => (error_status_code, Json(ApiResponse::error(err.to_string()))).into_response(),
    }
}
