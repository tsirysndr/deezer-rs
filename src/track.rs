use crate::album::Album;
use crate::artist::Artist;
use crate::BASE_URL;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: i64,
    pub readable: Option<bool>,
    pub title: String,
    pub title_short: String,
    pub title_version: Option<String>,
    pub isrc: Option<String>,
    pub link: Option<String>,
    pub duration: u32,
    pub rank: Option<u32>,
    pub explicit_lyrics: bool,
    pub explicit_content_lyrics: Option<u32>,
    pub explicit_content_cover: Option<u32>,
    pub preview: String,
    pub artist: Artist,
    pub album: Option<Album>,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Tracks {
    pub data: Vec<Track>,
}

pub struct TrackService {
    client: Client,
}

impl TrackService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get(&self, id: &str) -> Result<Track, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/track/{}", id))
            .send()
            .await?
            .json()
            .await
    }
}
