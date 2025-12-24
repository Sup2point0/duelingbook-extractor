use anyhow as ah;

use crate::types::db::CardData;
use super::{
    MonsterCard,
    SpellCard,
    TrapCard,
};


#[derive(serde::Serialize, Clone, Debug)]
pub enum Card
{
    Monster(MonsterCard),
    Spell(SpellCard),
    Trap(TrapCard),
}

impl Card
{
    pub fn id(&self) -> u32
    {
        match self {
            Self::Monster(card) => card.id,
            Self::Spell(card)   => card.id,
            Self::Trap(card)    => card.id,
        }
    }
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
            Card::Spell(card)
                => write!(f,
                    "Spell{{ name: {}, property: {} }}",
                    card.name, card.property
                ),
            Card::Trap(card)
                => write!(f,
                    "Trap{{ name: {}, property: {} }}",
                    card.name, card.property
                ),
        }
    }
}

impl TryFrom<CardData> for Card
{
    type Error = ah::Error;

    fn try_from(data: CardData) -> Result<Self, Self::Error> {
        match data.card_type.as_str() {
            "Monster" => Ok(Card::Monster(MonsterCard::try_from(data)?)),
            "Spell"   => Ok(  Card::Spell(  SpellCard::try_from(data)?)),
            "Trap"    => Ok(   Card::Trap(   TrapCard::try_from(data)?)),
            _ => Err(ah::anyhow!("Invalid card type encountered: `{}`", data.card_type)),
        }
    }
}
