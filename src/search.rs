use crate::album::{Album, Albums};
use crate::artist::{Artist, Artists};
use crate::playlist::Playlists;
use crate::radio::Radios;
use crate::track::Tracks;
use serde::Deserialize;
use surf::Client;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
  pub id: u64,
  pub readable: bool,
  pub title: String,
  pub title_short: String,
  pub title_version: Option<String>,
  pub link: String,
  pub duration: u64,
  pub rank: u64,
  pub explicit_lyrics: bool,
  pub preview: String,
  pub artist: Artist,
  pub album: Album,
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
  pub data: Vec<SearchResult>,
}

pub struct SearchService {
  client: Client,
}

impl SearchService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get(&self, q: &str) -> Result<SearchResults, surf::Error> {
    let res = self
      .client
      .get(format!("/search?q={}", q))
      .recv_json::<SearchResults>()
      .await?;
    Ok(res)
  }

  pub async fn get_albums(&self, q: &str) -> Result<Albums, surf::Error> {
    let res = self
      .client
      .get(format!("/search/album?q={}", q))
      .recv_json::<Albums>()
      .await?;
    Ok(res)
  }

  pub async fn get_artists(&self, q: &str) -> Result<Artists, surf::Error> {
    let res = self
      .client
      .get(format!("/search/artist?q={}", q))
      .recv_json::<Artists>()
      .await?;
    Ok(res)
  }

  pub async fn get_playlists(&self, q: &str) -> Result<Playlists, surf::Error> {
    let res = self
      .client
      .get(format!("/search/playlist?q={}", q))
      .recv_json::<Playlists>()
      .await?;
    Ok(res)
  }

  pub async fn get_radio(&self, q: &str) -> Result<Radios, surf::Error> {
    let res = self
      .client
      .get(format!("/search/radio?q={}", q))
      .recv_json::<Radios>()
      .await?;
    Ok(res)
  }

  pub async fn get_tracks(&self, q: &str) -> Result<Tracks, surf::Error> {
    let res = self
      .client
      .get(format!("/search/track?q={}", q))
      .recv_json::<Tracks>()
      .await?;
    Ok(res)
  }
}
