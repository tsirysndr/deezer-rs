use surf::Client;

pub struct OptionsService {
    client: Client,
}

impl OptionsService {
    pub fn new(client: *const Client) -> Self {
        Self {
            client: unsafe { (*client).clone() },
        }
    }
}
