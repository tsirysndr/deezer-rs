use surf::Client;

pub struct RadioService {
    client: Client,
}

impl RadioService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
