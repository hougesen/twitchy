#[derive(Debug, serde::Deserialize)]
pub struct StartCommercialResponseData {
    /// The length of the commercial you requested. If you request a commercial that’s longer than 180 seconds, the API uses 180 seconds.
    pub length: u8,
    /// A message that indicates whether Twitch was able to serve an ad.
    pub message: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct StartCommercialResponse {
    /// An array that contains a single object with the status of your start commercial request.
    pub data: Vec<StartCommercialResponseData>,
    /// The number of seconds you must wait before running another commercial.  
    pub retry_after: i32,
}
