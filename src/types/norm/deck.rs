use super::Card;


#[derive(std::fmt::Debug)]
pub struct Deck
{
    pub id: u32,
    pub name: String,

    pub main: Vec<Card>,
    pub side: Vec<Card>,
    pub extra: Vec<Card>,
}
