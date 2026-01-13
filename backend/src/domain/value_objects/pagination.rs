use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination<T: Serialize> {
    pub current_page: u32,
    pub found_pages: u32,
    pub has_next_page: bool,
    pub media: Vec<T>,
}
