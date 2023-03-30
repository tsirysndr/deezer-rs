use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub link: String,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub tracklist: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    pub data: Vec<User>,
}

pub struct UserService {
    #[allow(dead_code)]
    client: Client,
}

impl UserService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
