use crate::track::Tracks;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Radio {
    pub id: u64,
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

    pub async fn by_genre(&self) -> Result<Radios, surf::Error> {
        let res = self
            .client
            .get("/radio/genres")
            .recv_json::<Radios>()
            .await?;
        Ok(res)
    }

    pub async fn get_top_radio(&self) -> Result<Radios, surf::Error> {
        let res = self.client.get("/radio/top").recv_json::<Radios>().await?;
        Ok(res)
    }

    pub async fn get_tracks(&self, id: &str) -> Result<Tracks, surf::Error> {
        let res = self
            .client
            .get(format!("/radio/{}/tracks", id))
            .recv_json::<Tracks>()
            .await?;
        Ok(res)
    }

    pub async fn list(&self) -> Result<Radios, surf::Error> {
        let res = self
            .client
            .get("/radio/lists")
            .recv_json::<Radios>()
            .await?;
        Ok(res)
    }
}
