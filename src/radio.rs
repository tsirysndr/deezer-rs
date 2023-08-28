use crate::track::Tracks;
use crate::BASE_URL;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Radio {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub share: Option<String>,
    pub picture: Option<String>,
    pub picture_small: Option<String>,
    pub picture_medium: Option<String>,
    pub picture_big: Option<String>,
    pub picture_xl: Option<String>,
    pub tracklist: Option<String>,
    pub md5_image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Radios {
    pub data: Vec<Radio>,
}

pub struct RadioService {
    client: Client,
}

impl RadioService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn by_genre(&self) -> Result<Radios, reqwest::Error> {
        self.client.get("/radio/genres").send().await?.json().await
    }

    pub async fn get_top_radio(&self) -> Result<Radios, reqwest::Error> {
        self.client.get("/radio/top").send().await?.json().await
    }

    pub async fn get_tracks(&self, id: &str) -> Result<Tracks, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/radio/{}/tracks", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list(&self) -> Result<Radios, reqwest::Error> {
        self.client.get("/radio/lists").send().await?.json().await
    }
}
