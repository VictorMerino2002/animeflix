use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeFlvApiResponse<T> {
    pub data: T,
    pub success: bool,
}
