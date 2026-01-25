use reqwest::Client;
use url::Url;

use crate::domain::entities::{Anime, Episode};
use crate::domain::value_objects::{Filters, Pagination};
use crate::infrastructure::value_objects::AnimeFlvApiResponse;

#[derive(Debug, thiserror::Error)]
pub enum HttpError {
    #[error("http error")]
    Http(#[from] reqwest::Error),

    #[error("url error")]
    Url(#[from] url::ParseError),
}

pub struct AnimeFlvClient {
    client: Client,
    base_url: Url,
}

impl AnimeFlvClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: Url::parse("https://animeflv.ahmedrangel.com/api/")
                .expect("base url inválida"),
        }
    }

    pub async fn get_anime_by_slug(&self, slug: &str) -> Result<Anime, HttpError> {
        let url = self.base_url.join("anime/")?.join(slug)?;

        let response: AnimeFlvApiResponse<Anime> =
            self.client.get(url).send().await?.json().await?;

        Ok(response.data)
    }

    pub async fn get_episode_by_slug(&self, slug: &str) -> Result<Episode, HttpError> {
        let url = self.base_url.join("anime/episode/")?.join(slug)?;

        let response: AnimeFlvApiResponse<Episode> =
            self.client.get(url).send().await?.json().await?;

        Ok(response.data)
    }

    pub async fn search_anime(
        &self,
        query: &str,
        page: u32,
    ) -> Result<Pagination<Anime>, HttpError> {
        let mut url = self.base_url.join("search")?;

        url.query_pairs_mut()
            .append_pair("query", query)
            .append_pair("page", &page.to_string());

        let response: AnimeFlvApiResponse<Pagination<Anime>> =
            self.client.get(url).send().await?.json().await?;

        Ok(response.data)
    }

    pub async fn get_on_air_animes(&self) -> Result<Vec<Anime>, HttpError> {
        let url = self.base_url.join("list/animes-on-air")?;

        let response: AnimeFlvApiResponse<Vec<Anime>> =
            self.client.get(url).send().await?.json().await?;

        Ok(response.data)
    }

    pub async fn search_anime_by_filters(
        &self,
        filters: Option<Filters>,
        page: u32,
        order: &str,
    ) -> Result<Pagination<Anime>, HttpError> {
        let mut url = self.base_url.join("search/by-filter")?;

        url.query_pairs_mut()
            .append_pair("order", order)
            .append_pair("page", &page.to_string());

        let mut request_builder = self
            .client
            .post(url)
            .header("Content-Type", "application/json");

        if let Some(f) = filters {
            request_builder = request_builder.json(&f);
        }

        let response: AnimeFlvApiResponse<Pagination<Anime>> =
            request_builder.send().await?.json().await?;

        Ok(response.data)
    }
}
