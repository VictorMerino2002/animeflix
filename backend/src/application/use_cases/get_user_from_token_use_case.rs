use std::sync::Arc;

use crate::{
    domain::{entities::User, value_objects::Error},
    infrastructure::adapters::{JwtEncoder, SqlUserRepository},
};

pub struct GetUserFromTokenUseCase {
    pub user_repository: Arc<SqlUserRepository>,
    pub jwt_encoder: Arc<JwtEncoder>,
}

impl GetUserFromTokenUseCase {
    pub async fn execute(&self, token: &str) -> Result<User, Error> {
        let jwt = self
            .jwt_encoder
            .decode(token)
            .map_err(|_| Error::unauthorized("Invalid auth token"))?;

        let user = self
            .user_repository
            .get_by_username(&jwt.sub)
            .await
            .map_err(|_| Error::internal_server("Error getting user"))?;

        let user = user.ok_or_else(|| Error::unauthorized("Invalid auth token"))?;

        Ok(user)
    }
}
