use std::sync::Arc;

use crate::{
    domain::{entities::User, value_objects::Error},
    infrastructure::adapters::{ArgonPasswordEncriptor, SqlUserRepository},
};

pub struct CreateUserUseCase {
    pub user_repository: Arc<SqlUserRepository>,
    pub password_encryptor: Arc<ArgonPasswordEncriptor>,
}

impl CreateUserUseCase {
    pub async fn execute(&self, username: &str, password: &str) -> Result<User, Error> {
        let exists = self
            .user_repository
            .exist_by_username(username)
            .await
            .map_err(|_| Error::internal_server("Error creating user"))?;

        if exists {
            return Err(Error::bad_request("User already exists"));
        }

        let hashed_password = self.password_encryptor.hash_password(password);

        let mut user = User::new(username, &hashed_password);
        self.user_repository
            .create(&user)
            .await
            .map_err(|_| Error::internal_server("Error creating user"))?;
        user.password = None;
        Ok(user)
    }
}
