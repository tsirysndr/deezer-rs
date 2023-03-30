use reqwest::Client;

pub struct InfosService {
    #[allow(dead_code)]
    client: Client,
}

impl InfosService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
