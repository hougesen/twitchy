use crate::{error::TwitchyError, response::unwrap_twitch_response};

pub mod results;

#[derive(serde::Serialize)]
struct StartCommercialRequestBody<'a> {
    broadcaster_id: &'a str,
    commercial_length: i32,
}

impl crate::Twitchy {
    /// # Starts a commercial on the specified channel.
    ///
    /// NOTE: Only partners and affiliates may run commercials and they must be streaming live at the time.
    ///
    /// NOTE: Only the broadcaster may start a commercial; the broadcaster’s editors and moderators may not start commercials on behalf of the broadcaster.
    ///
    /// ## Authorization
    /// Requires a user access token that includes the channel:edit:commercial scope.
    ///
    /// ## Arguments
    /// * `broadcaster_id` - The ID of the partner or affiliate broadcaster that wants to run the commercial. This ID must match the user ID found in the OAuth token.
    /// * `commercial_length` - The length of the commercial to run, in seconds. Twitch tries to serve a commercial that’s the requested length, but it may be shorter or longer. The maximum length you should request is 180 seconds.
    pub async fn start_commercial(
        &self,
        broadcaster_id: &str,
        commercial_length: i32,
    ) -> Result<results::StartCommercialResponse, TwitchyError> {
        let request_body = StartCommercialRequestBody {
            broadcaster_id,
            commercial_length,
        };

        let response = self
            .http_client
            .post("https://api.twitch.tv/helix/channels/commercial")
            .json(&request_body)
            .send()
            .await;

        unwrap_twitch_response(response).await
    }
}
