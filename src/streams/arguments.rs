use typed_builder::TypedBuilder;

pub enum TwitchGetStreamsType {
    All,
    Live,
}

#[derive(Default, TypedBuilder)]
pub struct GetStreamsArguments<'a> {
    /// A user ID used to filter the list of streams. Returns only the streams of those users that are broadcasting. You may specify a maximum of 100 IDs.
    #[builder(default)]
    pub user_ids: Option<&'a [String]>,
    #[builder(default)]
    pub user_logins: Option<&'a [String]>,
    #[builder(default)]
    pub game_ids: Option<&'a [String]>,
    #[builder(default)]
    pub r#type: Option<TwitchGetStreamsType>,
    #[builder(default)]
    pub languages: Option<&'a [String]>,
    #[builder(default)]
    pub first: Option<u8>,
    #[builder(default)]
    pub before: Option<String>,
    #[builder(default)]
    pub after: Option<String>,
}
