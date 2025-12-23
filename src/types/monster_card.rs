use crate::types::CustomCard;


pub struct MonsterCard
{
    id: u32,
    name: String,
    username: String,

    is_effect: bool,
    effect: String,

    level: u8,
    attribute: String,
    r#type: String,
    ability: String,

    atk: String,
    def: String,

    pendulum: bool,
    scale: u8,
    pendulum_effect: String,

    arrows: String,

    tcg_limit: u8,
}

impl CustomCard for MonsterCard
{
    fn id(&self)      -> u32  { self.id }
    fn name(&self)    -> &str { &self.name }
    fn creator(&self) -> &str { &self.username }
}
