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

    pub fn get(&self, id: &str) {}

    pub fn list(&self) {}

    pub fn get_artists(&self, id: &str) {}
}
