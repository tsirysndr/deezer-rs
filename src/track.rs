use surf::Client;

pub struct TrackService {
    client: Client,
}

impl TrackService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
