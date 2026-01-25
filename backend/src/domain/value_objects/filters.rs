use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Filters {
    pub types: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub statuses: Option<Vec<i8>>,
}
