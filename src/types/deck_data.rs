use super::Card;


pub struct DeckData
{
    id: u32,
    name: String,

    main: Vec<Card>,
    side: Vec<Card>,
    extra: Vec<Card>,
}
