use crate::error::TwitchyError;

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum TwitchResponse<T> {
    Success(T),
    Failed(crate::error::TwitchErrorResponse),
}

pub async fn unwrap_twitch_response<T: for<'a> serde::Deserialize<'a>>(
    request: Result<reqwest::Response, reqwest::Error>,
) -> Result<T, TwitchyError> {
    match request {
        Ok(response) => match response.json::<TwitchResponse<T>>().await {
            Ok(json) => match json {
                TwitchResponse::Success(s) => Ok(s),
                TwitchResponse::Failed(e) => Err(unwrap_twitch_error(e)),
            },
            Err(error) => Err(TwitchyError::ReqwestError(error)),
        },
        Err(parse_error) => Err(TwitchyError::ReqwestError(parse_error)),
    }
}

fn unwrap_twitch_error(error: crate::error::TwitchErrorResponse) -> TwitchyError {
    let formatted_message = format!(
        "{} {} {}",
        error.status,
        error.error.unwrap_or_default(),
        error.message.unwrap_or_default()
    );

    match error.status {
        400 => TwitchyError::BadRequest(formatted_message),
        401 => TwitchyError::Unauthorized(formatted_message),
        403 => TwitchyError::Forbidden(formatted_message),
        404 => TwitchyError::NotFound(formatted_message),
        409 => TwitchyError::Conflict(formatted_message),
        429 => TwitchyError::TooManyRequests(formatted_message),
        _ => TwitchyError::UnknownError(formatted_message),
    }
}
