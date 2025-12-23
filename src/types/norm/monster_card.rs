use crate::types::db::CardData;


#[derive(std::fmt::Debug)]
pub struct MonsterCard
{
    pub id:      u32,
    pub name:    String,
    pub creator: String,

    pub is_effect: bool,
    pub effect:    String,

    pub monster_kind: monster::Kind,
    pub level:        u8,
    pub attribute:    monster::Attribute,
    pub monster_type: monster::Type,
    pub ability:      Vec<monster::Ability>,
    pub flip:         bool,

    pub atk: Option<u16>,
    pub def: Option<u16>,

    pub is_pend:     bool,
    pub pend_scale:  u8,
    pub pend_effect: String,

    pub link_arrows: Vec<monster::LinkArrow>,

    pub limit:   u8,
}

impl TryFrom<CardData> for MonsterCard
{
    type Error = Box<dyn std::error::Error>;

    fn try_from(data: CardData) -> Result<Self, Self::Error>
    {
        Ok(Self {
            id:      data.id,
            name:    data.name,
            creator: data.username,

            is_effect: data.is_effect > 0,
            effect:    data.effect,

            monster_kind: monster::Kind::try_from(data.monster_color)?,
            level:        data.level,
            attribute:    monster::Attribute::try_from(data.attribute)?,
            monster_type: monster::Type::try_from(data.r#type)?,
            ability:      monster::Ability::try_from_many(data.ability)?,
            flip:         data.flip > 0,
            atk:          monster::try_parse_atk_def(data.atk)?,
            def:          monster::try_parse_atk_def(data.def)?,
            is_pend:      data.pendulum > 0,
            pend_scale:   data.scale,
            pend_effect:  data.pendulum_effect,

            link_arrows:  monster::LinkArrow::try_from_many(data.arrows)?,

            limit:        data.tcg_limit,
        })
    }
}


pub use monster::*;

mod monster
{

#[derive(std::fmt::Debug)]
pub enum Kind {
    NORMAL, EFFECT, RITUAL,
    FUSION, SYNCHRO, XYZ, LINK,
}

impl TryFrom<String> for Kind {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Normal"  => Ok(Self::NORMAL),
            "Effect"  => Ok(Self::EFFECT),
            "Ritual"  => Ok(Self::RITUAL),
            "Fusion"  => Ok(Self::FUSION),
            "Synchro" => Ok(Self::SYNCHRO),
            "Xyz"     => Ok(Self::XYZ),
            "Link"    => Ok(Self::LINK),
            _ => Err(format!("Received invalid Monster Card Type: {value}")),
        }
    }
}


#[derive(std::fmt::Debug)]
pub enum Attribute {
    LIGHT, DARK, WATER, FIRE, EARTH, WIND, DIVINE,
}

impl TryFrom<String> for Attribute {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "LIGHT" => Ok(Self::LIGHT),
            "DARK"  => Ok(Self::DARK),
            "WATER" => Ok(Self::DARK),
            "FIRE"  => Ok(Self::FIRE),
            "EARTH" => Ok(Self::EARTH),
            "WIND"  => Ok(Self::WIND),
            _ => Err(format!("Received invalid Monster Attribute: {value}")),
        }
    }
}


#[derive(std::fmt::Debug)]
pub enum Type {
    CYBERSE, DRAGON, MACHINE,
}

impl TryFrom<String> for Type {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "cyberse" => Ok(Self::CYBERSE),
            "dragon"  => Ok(Self::DRAGON),
            "machine" => Ok(Self::MACHINE),
            _ => Err(format!("Received invalid Monster Type: {value}")),
        }
    }
}


#[derive(std::fmt::Debug)]
pub enum Ability {
    GEMINI, SPIRIT, TOON, TUNER, UNION,
}

impl Ability {
    pub fn try_from_many(value: String) -> Result<Vec<Self>, String> {
        value.split(r#" \/ "#)
            .map(|s| Self::try_from(s.to_string()))
            .collect()
    }
}

impl TryFrom<String> for Ability {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Gemini" => Ok(Self::GEMINI),
            "Spirit" => Ok(Self::SPIRIT),
            "Toon"   => Ok(Self::TOON),
            "Tuner"  => Ok(Self::TUNER),
            "Union"  => Ok(Self::UNION),
            _ => Err(format!("Received invalid Monster Ability: {value}")),
        }
    }
}


pub fn try_parse_atk_def(value: String) -> anyhow::Result<Option<u16>>
{
    if value == "?" {
        Ok(None)
    } else {
        Ok(Some(value.parse()?))
    }
}


#[derive(std::fmt::Debug)]
pub enum LinkArrow {
    UP, DOWN, LEFT, RIGHT,
    UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
}

impl LinkArrow {
    pub fn try_from_many(value: String) -> Result<Vec<Self>, String> {
        let out: Vec<Self> =
            value
            .chars()
            .zip(LINK_ARROWS)
            .filter(|(c, _)| *c == '1')
            .map(|(_, arrow)| arrow)
            .collect();

        if out.len() != 8 {
            Err(format!("Received unexpected number of Link Arrows: {}", out.len()))
        } else {
            Ok(out)
        }
    }
}

const LINK_ARROWS: [LinkArrow; 8] = [
    LinkArrow::UP_LEFT,
    LinkArrow::UP,
    LinkArrow::UP_RIGHT,
    LinkArrow::RIGHT,
    LinkArrow::DOWN_RIGHT,
    LinkArrow::DOWN,
    LinkArrow::DOWN_LEFT,
    LinkArrow::LEFT,
];

}
