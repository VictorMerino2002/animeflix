use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Episode {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub number: i32,
    #[serde(default)]
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    name: String,
    download: Option<String>,
    embed: Option<String>,
}
