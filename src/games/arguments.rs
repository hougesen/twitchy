use typed_builder::TypedBuilder;

use crate::utility::query::Querys;

#[derive(TypedBuilder)]
pub struct GetGamesArguments {
    #[builder(default)]
    game_ids: Option<Vec<String>>,
    #[builder(default)]
    names: Option<Vec<String>>,
    #[builder(default)]
    igdb_ids: Option<Vec<String>>,
}

impl From<GetGamesArguments> for Querys {
    fn from(value: GetGamesArguments) -> Self {
        let mut q = Self::new();

        if let Some(game_ids) = value.game_ids {
            if !game_ids.is_empty() {
                game_ids.iter().for_each(|id| q.insert("game_id", id));
            }
        }

        if let Some(names) = value.names {
            if !names.is_empty() {
                names.iter().for_each(|name| q.insert("name", name));
            }
        }

        if let Some(igdb_ids) = value.igdb_ids {
            if !igdb_ids.is_empty() {
                igdb_ids.iter().for_each(|id| q.insert("igdb_id", id));
            }
        }

        q
    }
}
