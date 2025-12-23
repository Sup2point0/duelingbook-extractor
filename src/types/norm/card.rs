use super::MonsterCard;
use crate::types::db::CardData;


#[derive(serde::Serialize, std::fmt::Debug)]
pub enum Card
{
    Monster(MonsterCard),
    // Spell(SpellCard),
    // Trap(TrapCard),
}

impl std::fmt::Display for Card
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Monster(card)
            => write!(f,
                "Monster{{ name: {}, level: {} }}",
                card.name, card.level
            ),
        }
    }
}

impl TryFrom<CardData> for Card
{
    type Error = anyhow::Error;

    fn try_from(data: CardData) -> Result<Self, Self::Error> {
        match data.card_type.as_str() {
            "Monster" => Ok(Card::Monster(MonsterCard::try_from(data)?)),
            // "Spell"   => Card::Spell(SpellCard::from(data)),
            // "Trap"    => Card::Trap(TrapCard::from(data)),
            _ => Err(anyhow::anyhow!("Invalid card type encountered: {}", data.card_type)),
        }
    }
}
