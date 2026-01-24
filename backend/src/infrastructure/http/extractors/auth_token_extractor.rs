use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};

use crate::{domain::value_objects::Error, infrastructure::http::ApiResponse};

pub struct AuthToken(pub String);

impl<S> FromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(auth) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    ApiResponse::default(Err(Error {
                        code: axum::http::StatusCode::UNAUTHORIZED,
                        message: "Authorization header missing or invalid".to_string(),
                    }))
                })?;

        Ok(AuthToken(auth.token().to_string()))
    }
}
