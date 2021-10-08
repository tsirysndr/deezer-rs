use surf::Client;

pub struct ArtistService {
  client: Client,
}

impl ArtistService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
