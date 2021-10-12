use surf::Client;

pub struct SearchService {
  client: Client,
}

impl SearchService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
