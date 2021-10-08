use surf::Client;

pub struct PlaylistService {
  client: Client,
}

impl PlaylistService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
