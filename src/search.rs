use crate::album::{Album, Albums};
use crate::artist::{Artist, Artists};
use crate::playlist::Playlists;
use crate::radio::Radios;
use crate::track::Tracks;
use crate::BASE_URL;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub readable: bool,
    pub title: String,
    pub title_short: String,
    pub title_version: String,
    pub link: String,
    pub duration: u64,
    pub rank: u64,
    pub explicit_lyrics: bool,
    pub preview: String,
    pub artist: Artist,
    pub album: Album,
    pub md5_image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    pub data: Vec<SearchResult>,
}

pub struct SearchService<'a> {
    client: &'a Client,
}

impl<'a> SearchService<'a> {
    pub fn new(client: &'a Client) -> SearchService<'a> {
        Self { client }
    }

    pub async fn get(&self, q: &str) -> Result<SearchResults, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/search?q={}", q))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_albums(&self, q: &str) -> Result<Albums, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/search/album?q={}", q))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_artists(&self, q: &str) -> Result<Artists, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/search/artist?q={}", q))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_playlists(&self, q: &str) -> Result<Playlists, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/search/playlist?q={}", q))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_radio(&self, q: &str) -> Result<Radios, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/search/radio?q={}", q))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_tracks(&self, q: &str) -> Result<Tracks, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/search/track?q={}", q))
            .send()
            .await?
            .json()
            .await
    }
}
