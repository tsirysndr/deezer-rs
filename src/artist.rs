use surf::Client;

pub struct ArtistService {
  client: Client,
}

impl ArtistService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn get(&self, id: &str) {}

  pub fn get_top_five(&self, id: &str) {}

  pub fn get_albums(&self, id: &str) {}

  pub fn get_playlists(&self, id: &str) {}

  pub fn get_fans(&self, id: &str) {}

  pub fn get_related(&self, id: &str) {}

  pub fn get_radio(&self, id: &str) {}

  pub fn get_comments(&self, id: &str) {}
}
