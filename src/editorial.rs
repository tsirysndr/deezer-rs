use surf::Client;

pub struct EditorialService {
  client: Client,
}

impl EditorialService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
