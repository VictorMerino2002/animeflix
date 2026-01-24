use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Jwt {
    pub sub: String,
    pub exp: usize,
}

impl Jwt {
    pub fn new(sub: &str) -> Self {
        let expiration = (Utc::now() + Duration::hours(24)).timestamp() as usize;

        Self {
            sub: sub.to_string(),
            exp: expiration,
        }
    }
}
