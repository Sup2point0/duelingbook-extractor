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
    type Error = &'static str;

    fn try_from(data: CardData) -> Result<Self, Self::Error>
    {
        Ok(Self {
            id:      data.id,
            name:    data.name,
            creator: data.username,

            is_effect: data.is_effect > 0,
            effect:    data.effect,

            monster_kind: monster::Kind::from(data.monster_color),
            level:        monster::check_level(data.level),
            attribute:    monster::Attribute::from(data.attribute),
            monster_type: monster::Type::from(data.r#type),
            ability:      monster::Ability::from_many(data.ability),
            flip:         data.flip > 0,
            atk:          data.atk.parse().ok(),
            def:          data.def.parse().ok(),
            is_pend:      data.pendulum > 0,
            pend_scale:   data.scale,
            pend_effect:  data.pendulum_effect,

            link_arrows:  monster::LinkArrow::from_many(data.arrows),

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

    impl From<String> for Kind {
        fn from(value: String) -> Self {
            match value.as_str() {
                "Normal"  => Self::NORMAL,
                "Effect"  => Self::EFFECT,
                "Ritual"  => Self::RITUAL,
                "Fusion"  => Self::FUSION,
                "Synchro" => Self::SYNCHRO,
                "Xyz"     => Self::XYZ,
                "Link"    => Self::LINK,
                _ => panic!("Received invalid Monster Card Type: {value}"),
            }
        }
    }


    pub fn check_level(level: u8) -> u8
    {
        assert!(level >= 0, "Received negative Monster Level: {level}");
        level
    }


    #[derive(std::fmt::Debug)]
    pub enum Attribute {
        LIGHT, DARK, WATER, FIRE, EARTH, WIND, DIVINE
    }

    impl From<String> for Attribute {
        fn from(value: String) -> Self {
            match value.as_str() {
                "LIGHT" => Self::LIGHT,
                "DARK"  => Self::DARK,
                "WATER" => Self::DARK,
                "FIRE"  => Self::FIRE,
                "EARTH" => Self::EARTH,
                "WIND"  => Self::WIND,
                _ => panic!("Received invalid Monster Attribute: {value}"),
            }
        }
    }


    #[derive(std::fmt::Debug)]
    pub enum Type {
        CYBERSE, DRAGON, MACHINE,
    }

    impl From<String> for Type {
        fn from(value: String) -> Self {
            match value.to_ascii_lowercase().as_str() {
                "cyberse" => Self::CYBERSE,
                "dragon"  => Self::DRAGON,
                "machine" => Self::MACHINE,
                _ => panic!("Received invalid Monster Type: {value}"),
            }
        }
    }


    #[derive(std::fmt::Debug)]
    pub enum Ability {
        GEMINI, SPIRIT, TOON, TUNER, UNION,
    }

    impl Ability {
        pub fn from_many(value: String) -> Vec<Self> {
            value.split(r#" \/ "#)
                .map(|s| Self::from(s.to_string()))
                .collect()
        }
    }

    impl From<String> for Ability {
        fn from(value: String) -> Self {
            match value.as_str() {
                "Gemini" => Self::GEMINI,
                "Spirit" => Self::SPIRIT,
                "Toon"   => Self::TOON,
                "Tuner"  => Self::TUNER,
                "Union"  => Self::UNION,
                _ => panic!("Received invalid Monster Ability: {value}"),
            }
        }
    }


    #[derive(std::fmt::Debug)]
    pub enum LinkArrow {
        UP, DOWN, LEFT, RIGHT,
        UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
    }

    impl LinkArrow {
        pub fn from_many(value: String) -> Vec<Self> {
            value.chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if c != '1' { None }
                    else { Some(match i {
                        0 => Self::UP_LEFT,
                        1 => Self::UP,
                        2 => Self::UP_RIGHT,
                        3 => Self::RIGHT,
                        4 => Self::DOWN_RIGHT,
                        5 => Self::DOWN,
                        6 => Self::DOWN_LEFT,
                        7 => Self::LEFT,
                        _ => panic!("Received more than 8 Link Arrows"),
                    }) }
                })
                .collect()
        }
    }
}
