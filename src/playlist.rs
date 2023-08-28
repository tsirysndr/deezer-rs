use crate::track::Tracks;
use crate::user::User;
use crate::BASE_URL;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub duration: Option<u32>,
    pub public: bool,
    pub is_loved_track: Option<bool>,
    pub collaborative: Option<bool>,
    pub nb_tracks: Option<u32>,
    pub unseen_track_count: Option<u32>,
    pub fans: Option<u32>,
    pub link: String,
    pub share: Option<String>,
    pub picture: String,
    pub picture_small: String,
    pub picture_medium: String,
    pub picture_big: String,
    pub picture_xl: String,
    pub checksum: String,
    pub creator: Option<User>,
    pub tracks: Option<Tracks>,
}

#[derive(Debug, Deserialize)]
pub struct Playlists {
    pub data: Vec<Playlist>,
}

pub struct PlaylistService {
    #[allow(dead_code)]
    client: Client,
}

impl PlaylistService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get(&self, id: &str) -> Result<Playlist, reqwest::Error> {
        // checks if id is numeric
        if !id.chars().all(|x| x.is_numeric()) {
            panic!("Playlist id is not a number")
        }
        self.client
            .get(format!("{BASE_URL}playlist/{}", id))
            .send()
            .await?
            .json::<Playlist>()
            .await
    }

    pub async fn get_comments(&self, _id: &str) -> Result<(), reqwest::Error> {
        Ok(())
    }

    pub async fn get_fans(&self, _id: &str) -> Result<(), reqwest::Error> {
        Ok(())
    }

    pub async fn get_tracks(&self, _id: &str) -> Result<(), reqwest::Error> {
        Ok(())
    }
}
