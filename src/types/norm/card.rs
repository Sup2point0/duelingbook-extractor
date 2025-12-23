use super::MonsterCard;


#[derive(serde::Deserialize, std::fmt::Debug)]
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
                "Monster{{ name: {}, level: {}, type: {}, attribute: {} }}",
                card.name, card.level, card.r#type, card.attribute
            ),
        }
    }
}


pub trait CustomCard
{
    fn id(&self)      -> u32;
    fn name(&self)    -> &str;
    fn creator(&self) -> &str;
}
