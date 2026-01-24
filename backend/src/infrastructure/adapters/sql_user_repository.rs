use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::domain::entities::User;

pub struct SqlUserRepository {
    pool: PgPool,
}

impl SqlUserRepository {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn create(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO users (uuid, username, password)
        VALUES ($1, $2, $3)
        "#,
            user.uuid,
            user.username,
            user.password
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn exist_by_username(&self, username: &str) -> Result<bool, sqlx::Error> {
        let option = sqlx::query!("SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(option.is_some())
    }
}
