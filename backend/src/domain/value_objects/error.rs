use axum::http::StatusCode;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub code: StatusCode,
    pub message: String,
}

impl Error {
    pub fn new(code: StatusCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }

    pub fn internal_server(message: &str) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_string(),
        }
    }

    pub fn not_found(message: &str) -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: message.to_string(),
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED,
            message: message.to_string(),
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for Error {}
