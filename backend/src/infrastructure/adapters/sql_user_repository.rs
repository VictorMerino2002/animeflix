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
}
