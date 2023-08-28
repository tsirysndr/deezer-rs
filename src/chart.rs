use crate::album::Albums;
use crate::artist::Artists;
use crate::playlist::Playlists;
use crate::track::Tracks;
use reqwest::Client;

pub struct ChartService {
    client: Client,
}

impl ChartService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get_tracks(&self) -> Result<Tracks, reqwest::Error> {
        self.client
            .get("/chart/0/tracks")
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_albums(&self) -> Result<Albums, reqwest::Error> {
        self.client
            .get("/chart/0/albums")
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_artists(&self) -> Result<Artists, reqwest::Error> {
        self.client
            .get("/chart/0/artists")
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_playlists(&self) -> Result<Playlists, reqwest::Error> {
        self.client
            .get("/chart/0/playlists")
            .send()
            .await?
            .json()
            .await
    }
}
