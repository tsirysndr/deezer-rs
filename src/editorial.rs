use surf::Client;

pub struct EditorialService {
  client: Client,
}

impl EditorialService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
