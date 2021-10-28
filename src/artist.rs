use crate::album::Albums;
use crate::playlist::Playlists;
use crate::track::Tracks;
use crate::user::Users;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct Artist {
  pub id: u64,
  pub name: String,
  pub link: Option<String>,
  pub share: Option<String>,
  pub picture: String,
  pub picture_small: String,
  pub picture_medium: String,
  pub picture_big: String,
  pub picture_xl: String,
  pub nb_album: Option<u32>,
  pub nb_fan: Option<u32>,
  pub radio: Option<bool>,
  pub tracklist: String,
  pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct Artists {
  pub data: Vec<Artist>,
}

pub struct ArtistService {
  client: Client,
}

impl ArtistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self, id: &str) -> Result<Artist, surf::Error> {
    let res = self
      .client
      .get(format!("/artist/{}", id))
      .recv_json::<Artist>()
      .await?;
    Ok(res)
  }

  pub async fn get_top_five(&self, id: &str) -> Result<Tracks, surf::Error> {
    let res = self
      .client
      .get(format!("/artist/{}/top", id))
      .recv_json::<Tracks>()
      .await?;
    Ok(res)
  }

  pub async fn get_albums(&self, id: &str) -> Result<Albums, surf::Error> {
    let res = self
      .client
      .get(format!("/artist/{}/albums", id))
      .recv_json::<Albums>()
      .await?;
    Ok(res)
  }

  pub async fn get_playlists(&self, id: &str) -> Result<Playlists, surf::Error> {
    let res = self
      .client
      .get(format!("/artist/{}/playlists", id))
      .recv_json::<Playlists>()
      .await?;
    Ok(res)
  }

  pub async fn get_fans(&self, id: &str) -> Result<Users, surf::Error> {
    let res = self
      .client
      .get(format!("/artist/{}/fans", id))
      .recv_json::<Users>()
      .await?;
    Ok(res)
  }

  pub async fn get_related(&self, id: &str) -> Result<Artists, surf::Error> {
    let res = self
      .client
      .get(format!("/artist/{}/related", id))
      .recv_json::<Artists>()
      .await?;
    Ok(res)
  }

  pub async fn get_radio(&self, id: &str) -> Result<Tracks, surf::Error> {
    let res = self
      .client
      .get(format!("/artist/{}/radio", id))
      .recv_json::<Tracks>()
      .await?;
    Ok(res)
  }
}
