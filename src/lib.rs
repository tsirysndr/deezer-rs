use reqwest::Client;
use std::time::Duration;

pub mod album;
pub mod artist;
pub mod chart;
pub mod editorial;
pub mod genre;
pub mod infos;
pub mod options;
pub mod playlist;
pub mod radio;
pub mod search;
pub mod track;
pub mod user;

pub use reqwest::Error;

#[derive(Clone)]
pub struct Deezer {
    client: Client,
}

pub const BASE_URL: &str = "https://api.deezer.com/";

impl Deezer {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        Self { client }
    }

    pub fn track(&self) -> track::TrackService {
        track::TrackService::new(&self.client)
    }

    pub fn search(&self) -> search::SearchService {
        search::SearchService::new(&self.client)
    }

    pub fn playlist(&self) -> playlist::PlaylistService {
        playlist::PlaylistService::new(&self.client)
    }
}

impl Default for Deezer {
    fn default() -> Self {
        Self::new()
    }
}
