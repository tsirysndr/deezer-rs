use crate::track::Tracks;
use crate::user::Users;
use crate::BASE_URL;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub upc: Option<String>,
    pub link: Option<String>,
    pub share: Option<String>,
    pub cover: String,
    pub cover_small: String,
    pub cover_medium: String,
    pub cover_big: String,
    pub cover_xl: String,
    pub genre_id: Option<i64>,
    pub label: Option<String>,
    pub nb_tracks: Option<u32>,
    pub duration: Option<u32>,
    pub fans: Option<u32>,
    pub rating: Option<u32>,
    pub release_date: Option<String>,
    pub record_type: Option<String>,
    pub available: Option<bool>,
    pub tracklist: String,
    pub explicit_lyrics: Option<bool>,
    pub explicit_content_lyrics: Option<u32>,
    pub explicit_content_cover: Option<u32>,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Albums {
    pub data: Vec<Album>,
}

pub struct AlbumService {
    client: Client,
}

impl AlbumService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get(&self, id: &str) -> Result<Album, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/album/{}", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_fans(&self, id: &str) -> Result<Users, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/album/{}/fans", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_tracks(&self, id: &str) -> Result<Tracks, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/album/{}/tracks", id))
            .send()
            .await?
            .json()
            .await
    }
}
