use surf::Client;

pub struct ChartService {
  client: Client,
}

impl ChartService {
  pub fn new(client: *const Client) -> Self {
    Self {
      client: unsafe { (*client).clone() },
    }
  }
}
