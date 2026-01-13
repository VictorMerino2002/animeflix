use serde::{Deserialize, Serialize};

use crate::domain::value_objects::ApiResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeFlvApiResponse<T> {
    pub data: T,
    pub success: bool,
}

impl<T: Serialize> From<AnimeFlvApiResponse<T>> for ApiResponse<T> {
    fn from(value: AnimeFlvApiResponse<T>) -> Self {
        ApiResponse::new(value.success, value.data)
    }
}
