use crate::types::MonsterCard;


pub enum Card
{
    Monster(MonsterCard),
    // Spell(SpellCard),
    // Trap(TrapCard),
}

pub trait CustomCard
{
    fn id(&self)      -> u32;
    fn name(&self)    -> &str;
    fn creator(&self) -> &str;
}
