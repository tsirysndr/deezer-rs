use surf::Client;

pub struct RadioService {
    client: Client,
}

impl RadioService {
    pub fn new(client: *const Client) -> Self {
        Self {
            client: unsafe { (*client).clone() },
        }
    }
}
