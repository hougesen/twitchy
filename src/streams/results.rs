#[derive(serde::Deserialize, serde::Serialize)]
pub struct GetStreamsData {
    /// An ID that identifies the stream. You can use this ID later to look up the video on demand (VOD).
    pub id: String,
    /// The ID of the user that’s broadcasting the stream.
    pub user_id: String,
    /// The user’s login name.
    pub user_login: String,
    /// The user’s display name.
    pub user_name: String,
    /// The ID of the category or game being played.
    pub game_id: String,
    /// The name of the category or game being played.
    pub game_name: String,
    /// The type of stream. Possible values are: live | ''
    /// If an error occurs, this field is set to an empty string.
    pub r#type: String,
    /// The stream’s title. Is an empty string if not set.
    pub title: String,
    /// The tags applied to the stream.
    pub tags: Vec<String>,
    /// The number of users watching the stream.
    pub viewer_count: i32,
    /// The UTC date and time (in RFC3339 format) of when the broadcast began.
    pub started_at: String,
    /// The language that the stream uses. This is an ISO 639-1 two-letter language code or other if the stream uses a language not in the list of [supported stream languages](https://help.twitch.tv/s/article/languages-on-twitch?language=en_US#streamlang).
    pub language: String,
    /// A URL to an image of a frame from the last 5 minutes of the stream. Replace the width and height placeholders in the URL ({width}x{height}) with the size of the image you want, in pixels.
    pub thumbnail_url: String,
    /// A Boolean value that indicates whether the stream is meant for mature audiences.
    pub is_mature: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GetStreamsPagination {
    /// The cursor used to get the next page of results. Set the request’s after or before query parameter to this value depending on whether you’re paging forwards or backwards.
    pub cursor: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GetStreamsResponse {
    /// The list of streams.
    pub data: Vec<GetStreamsData>,
    /// The information used to page through the list of results. The object is empty if there are no more pages left to page through   
    pub pagination: GetStreamsPagination,
}
