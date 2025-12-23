use super::CardData;


#[derive(serde::Deserialize)]
pub struct DeckData
{
    pub id: u32,
    pub name: String,

    pub main: Vec<CardData>,
    pub side: Vec<CardData>,
    pub extra: Vec<CardData>,
}
