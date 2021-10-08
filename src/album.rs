use surf::Client;

pub struct AlbumService {
  client: Client,
}

impl AlbumService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
