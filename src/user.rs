use crate::playlist::Playlists;
use crate::BASE_URL;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub link: Option<String>,
    pub picture: Option<String>,
    pub picture_small: Option<String>,
    pub picture_medium: Option<String>,
    pub picture_big: Option<String>,
    pub picture_xl: Option<String>,
    pub tracklist: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    pub data: Vec<User>,
}

pub struct UserService {
    client: Client,
}

impl UserService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get(&self, id: &str) -> Result<User, reqwest::Error> {
        if !id.chars().all(|x| x.is_numeric()) {
            panic!("Playlist id is not a number")
        }
        self.client
            .get(format!("{BASE_URL}user/{}", id))
            .send()
            .await?
            .json::<User>()
            .await
    }

    pub async fn playlists(&self, id: &str) -> Result<Playlists, reqwest::Error> {
        if !id.chars().all(|x| x.is_numeric()) {
            panic!("Playlist id is not a number")
        }
        self.client
            .get(format!("{BASE_URL}user/{}/playlists", id))
            .send()
            .await?
            .json::<Playlists>()
            .await
    }
}
