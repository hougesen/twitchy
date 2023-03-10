pub mod error;
mod utility;

// twitch api
mod ads;
mod analytics;
mod authentication;
mod bits;
mod channel_points;
mod channels;
mod charity;
mod chat;
mod clips;
mod entitlements;
mod eventsub;
mod extensions;
pub mod games;
mod goals;
mod hype_train;
mod moderation;
mod music;
mod polls;
mod predictions;
mod raids;
mod response;
mod schedule;
mod search;
pub mod streams;
mod subscriptions;
mod tags;
mod teams;
mod users;
mod videos;
mod whispers;

pub struct Twitchy {
    pub client_id: String,
    pub client_secret: String,
    http_client: reqwest::Client,
}

impl Twitchy {
    pub async fn new(
        client_id: &str,
        client_secret: &str,
    ) -> Result<Twitchy, crate::error::TwitchyError> {
        Ok(Twitchy {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            http_client: crate::authentication::setup_http_client(client_id, client_secret).await?,
        })
    }

    pub fn set_client_id(&mut self, client_id: &str) {
        self.client_id = client_id.to_string()
    }

    pub fn set_client_secret(&mut self, client_id: &str) {
        self.client_id = client_id.to_string()
    }
}
