use std::str::FromStr;

use crate::{response::unwrap_twitch_response, utility::build_query_params};

pub mod arguments;
pub mod results;

impl crate::Twitchy {
    /// Gets the channel’s stream key.
    pub async fn get_stream_key(&self) {
        todo!()
    }

    /// Gets a list of all streams.
    pub async fn get_streams(
        &self,
        arguments: Option<arguments::GetStreamsArguments>,
    ) -> Result<results::GetStreamsResponse, crate::error::TwitchyError> {
        let mut base_url = url::Url::from_str("https://api.twitch.tv/helix/streams").unwrap();

        let mut query_params = String::new();

        if let Some(arguments) = arguments {
            if let Some(user_ids) = arguments.user_ids {
                if !user_ids.is_empty() {
                    let user_id_query = build_query_params(user_ids, "&user_id=");

                    if !user_id_query.is_empty() {
                        query_params.push_str("user_id=");

                        query_params.push_str(&user_id_query);
                    }
                }
            }

            if let Some(user_logins) = arguments.user_logins {
                if !user_logins.is_empty() {
                    let user_login_query = build_query_params(user_logins, "&user_login=");

                    if !user_login_query.is_empty() {
                        if !query_params.is_empty() {
                            query_params.push('&');
                        }

                        query_params.push_str("user_login=");

                        query_params.push_str(&user_login_query);
                    }
                }
            }

            if let Some(game_ids) = arguments.game_ids {
                if !game_ids.is_empty() {
                    let game_id_query = build_query_params(game_ids, "&game_id=");

                    if !game_id_query.is_empty() {
                        if !query_params.is_empty() {
                            query_params.push('&');
                        }

                        query_params.push_str("game_id=");

                        query_params.push_str(&game_id_query);
                    }
                }
            }

            if let Some(kind) = arguments.r#type {
                if !query_params.is_empty() {
                    query_params.push('&')
                }

                match kind {
                    arguments::TwitchGetStreamsType::All => query_params.push_str("type=all"),
                    arguments::TwitchGetStreamsType::Live => query_params.push_str("type=live"),
                }
            }

            if let Some(languages) = arguments.languages {
                if !languages.is_empty() {
                    let language_query = build_query_params(languages, "&language=");

                    if !language_query.is_empty() {
                        if !query_params.is_empty() {
                            query_params.push('&')
                        }

                        query_params.push_str("language=");

                        query_params.push_str(&language_query);
                    }
                }
            }

            if let Some(first) = arguments.first {
                if !query_params.is_empty() {
                    query_params.push('&');
                }

                query_params.push_str(&format!("first={first}"));
            }

            if let Some(before) = arguments.before {
                if !query_params.is_empty() {
                    query_params.push('&');
                }

                query_params.push_str(&format!("before={before}"));
            }

            if let Some(after) = arguments.after {
                if !query_params.is_empty() {
                    query_params.push('&');
                }

                query_params.push_str(&format!("after={after}"));
            }

            base_url.set_query(Some(&query_params));
        }

        unwrap_twitch_response(self.http_client.get(base_url).send().await).await
    }

    /// Gets the list of broadcasters that the user follows and that are streaming live.
    pub async fn get_followed_streams(&self) {
        todo!()
    }

    /// Adds a marker to a live stream.
    pub async fn create_stream_marker(&self) {
        todo!()
    }

    /// Gets a list of markers from the user’s most recent stream or from the specified VOD/video.
    pub async fn get_stream_markers(&self) {
        todo!()
    }
}
