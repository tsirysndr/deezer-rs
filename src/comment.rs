
use surf::Client;

pub struct CommentService {
    client: Client,
}

impl CommentService {
    pub fn new(client: *const Client) -> Self {
        Self {
            client: unsafe { (*client).clone() },
        }
    }
}
