use surf::Client;

pub struct TrackService {
    client: Client,
}

impl TrackService {
    pub fn new(client: *const Client) -> Self {
        Self {
            client: unsafe { (*client).clone() },
        }
    }
}
