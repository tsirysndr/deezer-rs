
use surf::Client;

pub struct UserService {
    client: Client,
}

impl UserService {
    pub fn new(client: *const Client) -> Self {
        Self {
            client: unsafe { (*client).clone() },
        }
    }
}
