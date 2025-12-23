type Bit = u8;


/// A generic card's (Monster, Spell or Trap) aggregate data, which will contain many invalid fields. Deserialised from DuelingBook's database response JSON.
#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct CardData
{
    /* NOTE: Field order follows DuelingBook's response JSON */
    pub id:        u32,
    pub name:      String,

    pub effect:    String,
    pub pendulum_effect: String,
    pub card_type: String,
    pub monster_color: String,
    pub is_effect: Bit,
    pub r#type:    String,
    pub attribute: String,
    pub level:     u8,
    pub ability:   String,
    pub flip:      Bit,
    pub pendulum:  Bit,
    pub scale:     u8,
    pub arrows:    String,
    pub atk:       String,
    pub def:       String,

    pub tcg_limit: u8,
    pub custom:    u8,
    pub username:  String,
}
