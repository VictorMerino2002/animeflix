use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response as AxumResponse};
use serde::Serialize;
use serde_json::json;

use crate::domain::value_objects::Error;

pub struct ApiResponse<T> {
    pub result: Result<T, Error>,
    pub code: StatusCode,
    pub success_on_error: bool,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> AxumResponse {
        match self.result {
            Ok(payload) => (
                self.code,
                Json(json!({
                    "success": true,
                    "payload": payload
                })),
            )
                .into_response(),

            Err(error) => {
                let status = if self.success_on_error {
                    StatusCode::OK
                } else {
                    error.code
                };

                (
                    status,
                    Json(json!({
                        "success": false,
                        "message": error.message
                    })),
                )
                    .into_response()
            }
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn default(result: Result<T, Error>) -> Self {
        Self {
            result,
            code: StatusCode::OK,
            success_on_error: false,
        }
    }

    pub fn with_code(mut self, code: StatusCode) -> Self {
        self.code = code;
        self
    }

    pub fn success_on_error(mut self, value: bool) -> Self {
        self.success_on_error = value;
        self
    }
}
