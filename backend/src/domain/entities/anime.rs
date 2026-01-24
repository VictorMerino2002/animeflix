use serde::{Deserialize, Serialize};

use crate::domain::entities::Episode;

#[derive(Debug, Deserialize, Serialize)]
pub struct Anime {
    pub slug: Option<String>,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    #[serde(default)]
    pub alternative_titles: Vec<String>,
    #[serde(default)]
    pub cover: String,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub rating: String,
    pub status: Option<String>,
    #[serde(default)]
    pub synopsis: String,
    pub next_airing_episode: Option<String>,
    #[serde(default)]
    pub related: Vec<AnimeRelation>,
    #[serde(default)]
    pub episodes: Vec<Episode>,
    pub last_episode_number_seen: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeRelation {
    slug: String,
    title: String,
    url: String,
    relation: String,
}
