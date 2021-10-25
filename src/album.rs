use crate::track::Tracks;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Album {
  pub id: u32,
  pub title: String,
  pub upc: String,
  pub link: String,
  pub share: String,
  pub cover: String,
  pub cover_small: String,
  pub cover_medium: String,
  pub cover_big: String,
  pub cover_xl: String,
  pub genre_id: u32,
  pub label: String,
  pub nb_tracks: u32,
  pub duration: u32,
  pub fans: u32,
  pub rating: Option<u32>,
  pub release_date: String,
  pub record_type: String,
  pub available: bool,
  pub tracklist: String,
  pub explicit_lyrics: bool,
  pub explicit_content_lyrics: u32,
  pub explicit_content_cover: u32,
  pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Fan {
  pub id: u32,
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
pub struct Fans {
  pub data: Vec<Fan>,
}

pub struct AlbumService {
  client: Client,
}

impl AlbumService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self, id: &str) -> Result<Album, surf::Error> {
    let res = self
      .client
      .get(format!("/album/{}", id))
      .recv_json::<Album>()
      .await?;
    Ok(res)
  }

  pub async fn get_fans(&self, id: &str) -> Result<Fans, surf::Error> {
    let res = self
      .client
      .get(format!("/album/{}/fans", id))
      .recv_json::<Fans>()
      .await?;
    Ok(res)
  }

  pub async fn get_tracks(&self, id: &str) -> Result<Tracks, surf::Error> {
    let res = self
      .client
      .get(format!("/album/{}/tracks", id))
      .recv_json::<Tracks>()
      .await?;
    Ok(res)
  }
}
