#[derive(Debug, serde::Deserialize)]
pub struct TwitchApiError {
    pub status: i16,
    pub message: String,
}

impl std::error::Error for TwitchApiError {}

impl std::fmt::Display for TwitchApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Twitch request failed with status code {} and error message '{}'",
            &self.status, &self.message
        )
    }
}
#[derive(Debug)]
pub enum TwitchyError {
    ReqwestError(reqwest::Error),
    TwitchError(TwitchApiError),
}
