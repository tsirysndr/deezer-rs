use surf::Client;

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
