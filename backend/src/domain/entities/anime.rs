use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Anime {
    pub slug: Option<String>,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    #[serde(default)]
    pub alternative_titles: Vec<String>,
    pub cover: String,
    #[serde(default)]
    pub genres: Vec<String>,
    pub rating: String,
    pub status: Option<String>,
    pub synopsis: String,
    pub next_airing_episode: Option<String>,
    #[serde(default)]
    pub related: Vec<AnimeRelation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeRelation {
    slug: String,
    title: String,
    url: String,
    relation: String,
}
