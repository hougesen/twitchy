#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetGamesResponseData {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub box_art_url: String,
    #[serde(default)]
    pub igdb_id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetGamesResponse {
    pub data: Vec<GetGamesResponseData>,
}
