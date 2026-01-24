use std::sync::Arc;

use crate::{
    domain::{entities::Jwt, value_objects::Error},
    infrastructure::adapters::{ArgonPasswordEncriptor, JwtEncoder, SqlUserRepository},
};

pub struct LoginUseCase {
    pub user_repository: Arc<SqlUserRepository>,
    pub password_encryptor: Arc<ArgonPasswordEncriptor>,
    pub jwt_encoder: Arc<JwtEncoder>,
}

impl LoginUseCase {
    pub async fn execute(&self, username: &str, password: &str) -> Result<String, Error> {
        let user = self
            .user_repository
            .get_by_username(username)
            .await
            .map_err(|_| Error::internal_server("Error trying to login"))?;

        let user = user.ok_or_else(|| Error::bad_request("Invalid Credentials"))?;

        let hashed_password = user
            .password
            .ok_or_else(|| Error::internal_server("Error trying to login"))?;

        if !self
            .password_encryptor
            .verify_password(&hashed_password, password)
        {
            return Err(Error::bad_request("Invalid Credentials"));
        }

        let jwt = Jwt::new(&user.username);

        let token = self
            .jwt_encoder
            .encode(&jwt)
            .map_err(|_| Error::internal_server("Error trying to login"))?;
        Ok(token)
    }
}
