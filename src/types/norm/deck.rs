use super::Card;
use crate::types::db::{CardData, DeckData};


#[derive(std::fmt::Debug)]
pub struct Deck
{
    pub id:   u32,
    pub name: String,

    pub main:  Vec<Card>,
    pub side:  Vec<Card>,
    pub extra: Vec<Card>,
}


impl From<DeckData> for Deck
{
    fn from(data: DeckData) -> Self {
        Self {
            id:   data.id,
            name: data.name,

            main:  from_cards_data(data.main),
            side:  from_cards_data(data.side),
            extra: from_cards_data(data.extra),
        }
    }
}


fn from_cards_data(data: Vec<CardData>) -> Vec<Card>
{
    data.into_iter()
        .filter_map(|cd| cd.try_into().ok())
        .collect()
}
