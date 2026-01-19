use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::domain::entities::{Episode, UserHistory};

pub struct SqlUserHistoryRepository {
    pool: PgPool,
}

impl SqlUserHistoryRepository {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn create(&self, user_history: &UserHistory) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO user_history (uuid, anime_slug, episode_slug, episode_num, user_uuid)
        VALUES ($1, $2, $3, $4, $5)
        "#,
            user_history.uuid,
            user_history.anime_slug,
            user_history.episode_slug,
            user_history.episode_num,
            user_history.user_uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_anime_history(
        &self,
        user_uuid: &str,
    ) -> Result<Vec<UserHistory>, sqlx::Error> {
        let anime_history = sqlx::query_as!(
            UserHistory,
            r#"
            SELECT DISTINCT ON (anime_slug)
                *
            FROM user_history
            WHERE user_uuid = $1
            ORDER BY anime_slug, created_at DESC;
            "#,
            user_uuid
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(anime_history)
    }

    pub async fn get_user_episode_history_by_anime(
        &self,
        anime_slug: &str,
        user_uuid: &str,
    ) -> Result<Vec<UserHistory>, sqlx::Error> {
        let history = sqlx::query_as!(
            UserHistory,
            r#"
        SELECT *
        FROM user_history
        WHERE 
        anime_slug = $1
        AND user_uuid = $2
        ORDER BY created_at DESC;
        "#,
            anime_slug,
            user_uuid
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(history)
    }
}
