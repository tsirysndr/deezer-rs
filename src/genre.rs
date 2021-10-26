use crate::artist::Artists;
use serde::Deserialize;
use surf::Client;

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

    pub async fn get(&self, id: &str) -> Result<Genre, surf::Error> {
        let res = self
            .client
            .get(format!("/genre/{}", id))
            .recv_json::<Genre>()
            .await?;
        Ok(res)
    }

    pub async fn list(&self) -> Result<Genres, surf::Error> {
        let res = self.client.get("/genre").recv_json::<Genres>().await?;
        Ok(res)
    }

    pub async fn get_artists(&self, id: &str) -> Result<Artists, surf::Error> {
        let res = self
            .client
            .get(format!("/genre/{}/artists", id))
            .recv_json::<Artists>()
            .await?;
        Ok(res)
    }
}
