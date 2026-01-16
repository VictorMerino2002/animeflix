use crate::domain::{entities::UserHistory, value_objects::Filter};
use mysql::{Pool, params, prelude::Queryable};
use std::error::Error;

pub struct MysqlUserHistoryRepository {
    pool: Pool,
}

impl MysqlUserHistoryRepository {
    pub fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = Pool::new(database_url)?;
        Ok(Self { pool })
    }

    pub fn create(&self, history: UserHistory) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        conn.exec_drop(
            "INSERT INTO user_history (uuid, anime_slug, episode_num, episode_slug, user_uuid)
             VALUES (:uuid, :anime_slug, :episode_num, :episode_slug, :user_uuid)",
            params! {
                "uuid" => &history.uuid,
                "anime_slug" => &history.anime_slug,
                "episode_num" => history.episode_num as i32,
                "episode_slug" => &history.episode_slug,
                "user_uuid" => &history.user_uuid,
            },
        )?;

        Ok(())
    }

    pub fn list_filtered(
        &self,
        filters: Vec<Filter>,
    ) -> Result<Vec<UserHistory>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;

        let mut sql =
            "SELECT uuid, anime_slug, episode_num, episode_slug, user_uuid FROM user_history"
                .to_string();
        let mut conditions: Vec<String> = Vec::new();
        let mut named_params: std::collections::HashMap<Vec<u8>, mysql::Value> =
            std::collections::HashMap::new();

        for (i, filter) in filters.into_iter().enumerate() {
            let param_name = format!("param{}", i);

            match filter {
                Filter::EqualsString(col, val) => {
                    conditions.push(format!("{} = :{}", col, param_name));
                    named_params.insert(param_name.into_bytes(), val.into());
                }
                Filter::NotEqualsString(col, val) => {
                    conditions.push(format!("{} != :{}", col, param_name));
                    named_params.insert(param_name.into_bytes(), val.into());
                }
                Filter::LessInt(col, val) => {
                    conditions.push(format!("{} < :{}", col, param_name));
                    named_params.insert(param_name.into_bytes(), (val as i32).into());
                }
                Filter::GreaterInt(col, val) => {
                    conditions.push(format!("{} > :{}", col, param_name));
                    named_params.insert(param_name.into_bytes(), (val as i32).into());
                }
            }
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let params = mysql::Params::Named(named_params);

        let result = conn.exec_map(
            sql,
            params,
            |(uuid, anime_slug, episode_num, episode_slug, user_uuid)| UserHistory {
                uuid,
                anime_slug,
                episode_slug,
                episode_num,
                user_uuid,
            },
        )?;

        Ok(result)
    }
}
