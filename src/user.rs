use surf::Client;

pub struct UserService {
    client: Client,
}

impl UserService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
