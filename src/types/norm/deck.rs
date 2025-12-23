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
            extra: from_cards_data(data.extra),
            side:  from_cards_data(data.side),
        }
    }
}

fn from_cards_data(data: Vec<CardData>) -> Vec<Card>
{
    data.into_iter()
        .filter_map(|cd| cd.try_into().ok())
        // .map(|cd| cd.try_into())
        .collect()
}


impl std::fmt::Display for Deck
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Deck {{ id: {}, name: {}, main: x{}, extra: x{} }}",
            self.id, self.name,
            self.main.len(), self.extra.len(),
        )
    }
}
