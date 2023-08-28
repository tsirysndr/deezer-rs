use crate::album::Albums;
use crate::playlist::Playlists;
use crate::track::Tracks;
use crate::user::Users;
use crate::BASE_URL;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub link: Option<String>,
    pub share: Option<String>,
    pub picture: Option<String>,
    pub picture_small: Option<String>,
    pub picture_medium: Option<String>,
    pub picture_big: Option<String>,
    pub picture_xl: Option<String>,
    pub nb_album: Option<u32>,
    pub nb_fan: Option<u32>,
    pub radio: Option<bool>,
    pub tracklist: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Artists {
    pub data: Vec<Artist>,
}

pub struct ArtistService {
    client: Client,
}

impl ArtistService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get(&self, id: &str) -> Result<Artist, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/artist/{}", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_top_five(&self, id: &str) -> Result<Tracks, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/artist/{}/top", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_albums(&self, id: &str) -> Result<Albums, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/artist/{}/albums", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_playlists(&self, id: &str) -> Result<Playlists, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/artist/{}/playlists", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_fans(&self, id: &str) -> Result<Users, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/artist/{}/fans", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_related(&self, id: &str) -> Result<Artists, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/artist/{}/related", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_radio(&self, id: &str) -> Result<Tracks, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/artist/{}/radio", id))
            .send()
            .await?
            .json()
            .await
    }
}
