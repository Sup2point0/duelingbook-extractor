use super::Card;


#[derive(serde::Deserialize)]
pub struct DeckData
{
    pub id: u32,
    pub name: String,

    pub main: Vec<Card>,
    pub side: Vec<Card>,
    pub extra: Vec<Card>,
}
