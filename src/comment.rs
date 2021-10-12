use surf::Client;

pub struct CommentService {
    client: Client,
}

impl CommentService {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}
