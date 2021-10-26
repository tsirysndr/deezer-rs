use crate::album::Albums;
use crate::artist::Artists;
use crate::playlist::Playlists;
use crate::track::Tracks;
use surf::Client;

pub struct ChartService {
  client: Client,
}

impl ChartService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub async fn get_tracks(&self) -> Result<Tracks, surf::Error> {
    let res = self
      .client
      .get("/chart/0/tracks")
      .recv_json::<Tracks>()
      .await?;
    Ok(res)
  }

  pub async fn get_albums(&self) -> Result<Albums, surf::Error> {
    let res = self
      .client
      .get("/chart/0/albums")
      .recv_json::<Albums>()
      .await?;
    Ok(res)
  }

  pub async fn get_artists(&self) -> Result<Artists, surf::Error> {
    let res = self
      .client
      .get("/chart/0/artists")
      .recv_json::<Artists>()
      .await?;
    Ok(res)
  }

  pub async fn get_playlists(&self) -> Result<Playlists, surf::Error> {
    let res = self
      .client
      .get("/chart/0/playlists")
      .recv_json::<Playlists>()
      .await?;
    Ok(res)
  }
}
