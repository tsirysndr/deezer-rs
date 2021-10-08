use surf::Client;

pub struct GenreService {
    client: Client,
}

impl GenreService {
    pub fn new(client: *const Client) -> Self {
        Self {
            client: unsafe { (*client).clone() },
        }
    }
}
