use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Track {
    pub id: u64,
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

    pub fn get(&self, id: &str) {}
}
