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

  pub fn get_tracks(&self) {}

  pub fn get_albums(&self) {}

  pub fn get_artists(&self) {}

  pub fn get_playlists(&self) {}
}
