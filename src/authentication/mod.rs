use crate::errors::{TwitchApiError, TwitchyError};

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
    let http_client = reqwest::Client::new();

    let uri = format!("https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",  client_id, client_secret);

    match http_client.post(uri).send().await {
        Ok(response) => {
            if response.status().is_success() {
                return response
                    .json::<GetTwitchAccessTokenResponse>()
                    .await
                    .map_err(|err| TwitchyError::ReqwestError(err));
            }

            let error_response = response.json::<TwitchApiError>().await;

            match error_response {
                Ok(twitch_error) => Err(TwitchyError::TwitchError(twitch_error)),
                Err(reqwest_error) => Err(TwitchyError::ReqwestError(reqwest_error)),
            }
        }
        Err(error) => Err(TwitchyError::ReqwestError(error)),
    }
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
        HeaderValue::from_str(&format!("Bearer {}", &token.access_token)).unwrap(),
    );

    headers.insert("Client-Id", HeaderValue::from_str(client_id).unwrap());

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| TwitchyError::ReqwestError(e))
}

impl crate::Twitchy {
    pub async fn refresh_authentication(&mut self) -> Result<(), TwitchyError> {
        self.http_client = setup_http_client(&self.client_id, &self.client_secret).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::{authentication::fetch_twitch_bearer_token, errors::TwitchyError, Twitchy};

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_valid_keys() {
        let client_id =
            dotenv::var("TWITCH_CLIENT_ID").expect("ERROR: Missing TWITCH_CLIENT_ID ENV");
        let client_secret =
            dotenv::var("TWITCH_CLIENT_SECRET").expect("ERROR: Missing TWITCH_CLIENT_SECRET ENV");

        let client = Twitchy::new(&client_id, &client_secret);

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_missing_client_id() {
        let client_id = "";

        let client_secret =
            dotenv::var("TWITCH_CLIENT_SECRET").expect("ERROR: Missing TWITCH_CLIENT_SECRET ENV");

        let client = Twitchy::new(client_id, &client_secret);

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_err());

        let error = authentication_result.expect_err("Authentication should have failed");

        assert!(matches!(error, TwitchyError::TwitchError { .. }));
    }

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_invalid_client_id() {
        let client_id = "this is not a real client id";

        let client_secret =
            dotenv::var("TWITCH_CLIENT_SECRET").expect("ERROR: Missing TWITCH_CLIENT_SECRET ENV");

        let client = Twitchy::new(client_id, &client_secret);

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_err());

        let error = authentication_result.expect_err("Authentication should have failed");

        assert!(matches!(error, TwitchyError::TwitchError { .. }));
    }

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_missing_client_secret() {
        let client_id =
            dotenv::var("TWITCH_CLIENT_ID").expect("ERROR: Missing TWITCH_CLIENT_ID ENV");

        let client_secret = "";

        let client = Twitchy::new(&client_id, &client_secret);

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_err());

        let error = authentication_result.expect_err("Authentication should have failed");

        assert!(matches!(error, TwitchyError::TwitchError { .. }));
    }

    #[tokio::test]
    async fn test_fetch_twitch_bearer_token_invalid_client_secret() {
        let client_id =
            dotenv::var("TWITCH_CLIENT_ID").expect("ERROR: Missing TWITCH_CLIENT_ID ENV");

        let client_secret = "this is not a real client secret";

        let client = Twitchy::new(&client_id, &client_secret);

        let authentication_result = fetch_twitch_bearer_token(&client_id, &client_secret).await;

        assert!(authentication_result.is_err());

        let error = authentication_result.expect_err("Authentication should have failed");

        assert!(matches!(error, TwitchyError::TwitchError { .. }));
    }
}
