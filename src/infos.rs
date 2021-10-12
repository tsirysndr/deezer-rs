use surf::Client;

pub struct InfosService {
    client: Client,
}

impl InfosService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
