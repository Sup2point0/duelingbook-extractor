use super::CustomCard;


#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct MonsterCard
{
    pub id: u32,
    pub name: String,
    pub username: String,

    pub is_effect: bool,
    pub effect: String,

    pub level: u8,
    pub attribute: String,
    pub r#type: String,
    pub ability: String,

    pub atk: String,
    pub def: String,

    pub pendulum: bool,
    pub scale: u8,
    pub pendulum_effect: String,

    pub arrows: String,

    pub tcg_limit: u8,
}

impl CustomCard for MonsterCard
{
    fn id(&self)      -> u32  { self.id }
    fn name(&self)    -> &str { &self.name }
    fn creator(&self) -> &str { &self.username }
}
