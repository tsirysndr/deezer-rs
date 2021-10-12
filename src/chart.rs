use surf::Client;

pub struct ChartService {
  client: Client,
}

impl ChartService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
