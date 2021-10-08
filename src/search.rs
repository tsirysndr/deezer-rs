use surf::Client;

pub struct SearchService {
  client: Client,
}

impl SearchService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
