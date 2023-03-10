use crate::{response::unwrap_twitch_response, utility::query::Querys};

use self::{arguments::GetGamesArguments, results::GetGamesResponse};

pub mod arguments;
pub mod results;

impl crate::Twitchy {
    /// Gets information about specified games.
    pub async fn get_games(
        &self,
        args: GetGamesArguments,
    ) -> Result<GetGamesResponse, crate::error::TwitchyError> {
        let mut base_url = "https://".parse::<url::Url>()?;

        let query = Querys::from(args);

        if !query.is_empty() {
            base_url.set_query(Some(&query.stringify()));
        }

        let request = self.http_client.get(base_url).send().await;

        unwrap_twitch_response(request).await
    }

    /// Gets information about all broadcasts on Twitch.
    pub async fn get_top_games(&self) {
        todo!()
    }
}
