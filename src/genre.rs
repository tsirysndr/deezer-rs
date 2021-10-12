use surf::Client;

pub struct GenreService {
    client: Client,
}

impl GenreService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
