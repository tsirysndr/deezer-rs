use surf::Client;

pub struct OptionsService {
    client: Client,
}

impl OptionsService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}

