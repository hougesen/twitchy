#[derive(Debug, serde::Deserialize)]
pub struct TwitchErrorResponse {
    pub status: i32,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl std::error::Error for TwitchErrorResponse {}

impl std::fmt::Display for TwitchErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Twitch error: request unsuccessfull. Status code: {}. Message: {:?}. Error: {:?}",
            &self.status, &self.message, &self.error
        )
    }
}

#[derive(Debug)]
pub struct InvalidArgumentError {
    pub argument: String,
    pub message: String,
}

#[derive(Debug)]
pub enum TwitchyError {
    ReqwestError(reqwest::Error),

    UrlParseError(url::ParseError),

    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),

    MissingClientId,
    MissingClientSecret,
    MissingAccessToken,
    InvalidParameters(String),

    // Twitch related
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    TooManyRequests(String),
    UnknownError(String),
}

impl std::error::Error for TwitchyError {}

impl std::fmt::Display for TwitchyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwitchyError::ReqwestError(e) => e.fmt(f),
            TwitchyError::BadRequest(message) => write!(f, "{}", message),
            TwitchyError::Unauthorized(message) => write!(f, "{}", message),
            TwitchyError::Forbidden(message) => write!(f, "{}", message),
            TwitchyError::NotFound(message) => write!(f, "{}", message),
            TwitchyError::TooManyRequests(message) => write!(f, "{}", message),
            TwitchyError::Conflict(message) => write!(f, "{}", message),
            TwitchyError::UnknownError(message) => write!(f, "{}", message),
            TwitchyError::MissingClientId => write!(f, "Twitchy error: Missing Twitch client id"),
            TwitchyError::MissingClientSecret => {
                write!(f, "Twitchy error: Missing Twitch client secret")
            }
            TwitchyError::MissingAccessToken => {
                write!(f, "Twitchy error: Missing Twitch access token")
            }
            TwitchyError::InvalidParameters(message) => {
                write!(f, "Twitchy error: Invalid parameters {}", message)
            }
            TwitchyError::UrlParseError(e) => e.fmt(f),
            TwitchyError::InvalidHeaderValue(e) => e.fmt(f),
        }
    }
}

impl From<reqwest::Error> for TwitchyError {
    fn from(value: reqwest::Error) -> Self {
        TwitchyError::ReqwestError(value)
    }
}

impl From<url::ParseError> for TwitchyError {
    fn from(value: url::ParseError) -> Self {
        TwitchyError::UrlParseError(value)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for TwitchyError {
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(value)
    }
}
