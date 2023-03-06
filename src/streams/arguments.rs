pub enum TwitchGetStreamsType {
    All,
    Live,
}

#[derive(Default)]
pub struct GetStreamsArguments {
    /// A user ID used to filter the list of streams. Returns only the streams of those users that are broadcasting. You may specify a maximum of 100 IDs.
    pub user_ids: Option<Vec<String>>,
    pub user_logins: Option<Vec<String>>,
    pub game_ids: Option<Vec<String>>,
    pub r#type: Option<TwitchGetStreamsType>,
    pub languages: Option<Vec<String>>,
    pub first: Option<u8>,
    pub before: Option<String>,
    pub after: Option<String>,
}

impl GetStreamsArguments {
    pub fn new(
        user_ids: Option<Vec<String>>,
        user_logins: Option<Vec<String>>,
        game_ids: Option<Vec<String>>,
        r#type: Option<TwitchGetStreamsType>,
        languages: Option<Vec<String>>,
        first: Option<u8>,
        before: Option<String>,
        after: Option<String>,
    ) -> Self {
        Self {
            user_ids,
            user_logins,
            game_ids,
            r#type,
            languages,
            first,
            before,
            after,
        }
    }
}
