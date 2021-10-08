use surf::Client;

pub struct InfosService {
    client: Client,
}

impl InfosService {
    pub fn new(client: *const Client) -> Self {
        Self {
            client: unsafe { (*client).clone() },
        }
    }
}
