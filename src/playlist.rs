use crate::track::Track;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Playlist {
  pub id: u64,
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
  pub tracks: Option<Vec<Track>>,
}

#[derive(Debug, Deserialize)]
pub struct Playlists {
  pub data: Vec<Playlist>,
}

pub struct PlaylistService {
  client: Client,
}

impl PlaylistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get(&self, id: &str) {}

  pub fn get_comments(&self, id: &str) {}

  pub fn get_fans(&self, id: &str) {}

  pub fn get_tracks(&self, id: &str) {}
}
