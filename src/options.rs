use reqwest::Client;

pub struct OptionsService {
    #[allow(dead_code)]
    client: Client,
}

impl OptionsService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
