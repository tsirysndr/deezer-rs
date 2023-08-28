use reqwest::Client;

pub struct EditorialService {
    #[allow(dead_code)]
    client: Client,
}

impl EditorialService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
