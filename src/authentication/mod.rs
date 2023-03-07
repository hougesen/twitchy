use crate::{error::TwitchyError, response::unwrap_twitch_response};

#[derive(Debug, serde::Deserialize)]
pub struct GetTwitchAccessTokenResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
}

pub(crate) async fn fetch_twitch_bearer_token(
    client_id: &str,
    client_secret: &str,
) -> Result<GetTwitchAccessTokenResponse, TwitchyError> {
    if client_id.is_empty() {
        return Err(TwitchyError::MissingClientId);
    }

    if client_secret.is_empty() {
        return Err(TwitchyError::MissingClientSecret);
    }

    let http_client = reqwest::Client::new();

    let uri = format!("https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",  client_id, client_secret);

    unwrap_twitch_response(http_client.post(uri).send().await).await
}

pub(crate) async fn setup_http_client(
    client_id: &str,
    client_secret: &str,
) -> Result<reqwest::Client, TwitchyError> {
    use reqwest::header::{HeaderMap, HeaderValue};

    let token = fetch_twitch_bearer_token(client_id, client_secret).await?;

    let mut headers = HeaderMap::new();

    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", &token.access_token))?,
    );

    headers.insert("Client-Id", HeaderValue::from_str(client_id)?);

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(TwitchyError::ReqwestError)
}

impl crate::Twitchy {
    pub async fn refresh_authentication(&mut self) -> Result<(), TwitchyError> {
        self.http_client = setup_http_client(&self.client_id, &self.client_secret).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::{authentication::fetch_twitch_bearer_token, error::TwitchyError};

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_valid_keys() {
        let client_id =
            dotenv::var("TWITCH_CLIENT_ID").expect("ERROR: Missing TWITCH_CLIENT_ID ENV");
        let client_secret =
            dotenv::var("TWITCH_CLIENT_SECRET").expect("ERROR: Missing TWITCH_CLIENT_SECRET ENV");

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_missing_client_id() {
        let client_id = "";

        let client_secret =
            dotenv::var("TWITCH_CLIENT_SECRET").expect("ERROR: Missing TWITCH_CLIENT_SECRET ENV");

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_err());

        let error = authentication_result.expect_err("Authentication should have failed");

        assert!(matches!(error, TwitchyError::MissingClientId));
    }

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_invalid_client_id() {
        let client_id = "this is not a real client id";

        let client_secret =
            dotenv::var("TWITCH_CLIENT_SECRET").expect("ERROR: Missing TWITCH_CLIENT_SECRET ENV");

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_err());

        let error = authentication_result.expect_err("Authentication should have failed");

        assert!(matches!(error, TwitchyError::BadRequest { .. }));
    }

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_missing_client_secret() {
        let client_id =
            dotenv::var("TWITCH_CLIENT_ID").expect("ERROR: Missing TWITCH_CLIENT_ID ENV");

        let client_secret = "";

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_err());

        let error = authentication_result.expect_err("Authentication should have failed");

        assert!(matches!(error, TwitchyError::MissingClientSecret { .. }));
    }

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_invalid_client_secret() {
        let client_id =
            dotenv::var("TWITCH_CLIENT_ID").expect("ERROR: Missing TWITCH_CLIENT_ID ENV");

        let client_secret = "this is not a real client secret";

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_err());

        let error = authentication_result.expect_err("Authentication should have failed");

        assert!(matches!(error, TwitchyError::Forbidden { .. }));
    }
}
