use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};

use crate::{
    application::use_cases::GetUserFromTokenUseCase,
    domain::{entities::User, value_objects::Error},
    infrastructure::{bootstrap::DepContianer, http::ApiResponse},
};

pub struct AuthUser(pub User);

impl FromRequestParts<Arc<DepContianer>> for AuthUser {
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<DepContianer>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(auth) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    ApiResponse::default(Err(Error::unauthorized(
                        "Authorization header missing or invalid",
                    )))
                })?;

        let use_case = GetUserFromTokenUseCase {
            user_repository: state.user_repository.clone(),
            jwt_encoder: state.jwt_encoder.clone(),
        };

        let user = use_case
            .execute(auth.token())
            .await
            .map_err(|err| ApiResponse::default(Err(err)))?;

        Ok(AuthUser(user))
    }
}
