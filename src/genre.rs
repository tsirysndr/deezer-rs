use crate::artist::Artists;
use crate::BASE_URL;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
}

#[derive(Debug, Deserialize)]
pub struct Genres {
    pub data: Vec<Genre>,
}

pub struct GenreService {
    client: Client,
}

impl GenreService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get(&self, id: &str) -> Result<Genre, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/genre/{}", id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list(&self) -> Result<Genres, reqwest::Error> {
        self.client.get("/genre").send().await?.json().await
    }

    pub async fn get_artists(&self, id: &str) -> Result<Artists, reqwest::Error> {
        self.client
            .get(format!("{BASE_URL}/genre/{}/artists", id))
            .send()
            .await?
            .json()
            .await
    }
}
