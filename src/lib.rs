use authentication::setup_http_client;
use error::{TwitchErrorResponse, TwitchyError};

pub mod ads;
pub mod authentication;
pub mod error;
pub mod response;

pub struct Twitchy {
    pub client_id: String,
    pub client_secret: String,
    http_client: reqwest::Client,
}

impl Twitchy {
    pub async fn new(client_id: &str, client_secret: &str) -> Result<Twitchy, TwitchyError> {
        Ok(Twitchy {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            http_client: setup_http_client(client_id, client_secret).await?,
        })
    }

    pub fn set_client_id(&mut self, client_id: &str) {
        self.client_id = client_id.to_string()
    }

    pub fn set_client_secret(&mut self, client_id: &str) {
        self.client_id = client_id.to_string()
    }
}
