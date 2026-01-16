use crate::domain::entities::User;
use crate::domain::value_objects::Filter;
use mysql::params;
use mysql::{Params, Pool, Value, prelude::Queryable};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct MysqlUserRepository {
    pool: Pool,
}

impl MysqlUserRepository {
    pub fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = Pool::new(database_url)?;
        Ok(Self { pool })
    }

    pub fn get(&self, uuid: &str) -> Result<User, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        let result: Option<(String, String)> = conn.exec_first(
            "SELECT uuid, username FROM users WHERE uuid = :uuid",
            params! { "uuid" => uuid },
        )?;

        match result {
            Some((uuid, username)) => Ok(User {
                uuid,
                username,
                password: None,
            }),
            None => Err("User not found".into()),
        }
    }

    pub fn list(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        let users: Vec<User> =
            conn.query_map("SELECT uuid, username FROM users", |(uuid, username)| {
                User {
                    uuid,
                    username,
                    password: None,
                }
            })?;

        Ok(users)
    }

    // CREATE
    pub fn create(&self, user: User) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        conn.exec_drop(
            "INSERT INTO users (uuid, username, password) VALUES (:uuid, :username, :password)",
            mysql::Params::Named({
                let mut map = HashMap::new();
                map.insert(b"uuid".to_vec(), Value::from(user.uuid));
                map.insert(b"username".to_vec(), Value::from(user.username));
                map.insert(
                    b"password".to_vec(),
                    Value::from(user.password.unwrap_or_default()),
                );
                map
            }),
        )?;

        Ok(())
    }

    pub fn list_filtered(&self, filters: Vec<Filter>) -> Result<Vec<User>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        let mut sql = "SELECT uuid, username FROM users".to_string();
        let mut conditions: Vec<String> = Vec::new();
        let mut named_params: HashMap<Vec<u8>, Value> = HashMap::new();

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
                _ => return Err("Invalid filter".into()),
            }
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let params = Params::Named(named_params);

        let result = conn.exec_map(sql, params, |(uuid, username)| User {
            uuid,
            username,
            password: None,
        })?;

        Ok(result)
    }
}
