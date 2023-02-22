pub mod authentication;
pub mod errors;

pub struct Twitchy {
    pub client_id: String,
    pub client_secret: String,
    http_client: Option<reqwest::Client>,
}

impl Twitchy {
    pub fn new(client_id: &str, client_secret: &str) -> Twitchy {
        Twitchy {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            http_client: None,
        }
    }

    pub fn set_client_id(&mut self, client_id: &str) {
        self.client_id = client_id.to_string()
    }

    pub fn set_client_secret(&mut self, client_id: &str) {
        self.client_id = client_id.to_string()
    }
}
