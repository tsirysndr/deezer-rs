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
}
