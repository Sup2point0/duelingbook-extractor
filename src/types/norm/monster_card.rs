use anyhow as ah;

use crate::types::db::CardData;


#[derive(serde::Serialize, PartialEq, Clone, Debug)]
pub struct MonsterCard
{
    pub id:      u32,
    pub name:    String,
    pub creator: String,

    pub is_effect: bool,
    pub effect:    String,

    pub kind:         monster::Kind,
    pub level:        u8,
    pub attribute:    monster::Attribute,
    pub monster_type: monster::Type,
    pub ability:      Vec<monster::Ability>,
    pub flip:         bool,

    pub atk: Option<u16>,
    pub def: Option<u16>,

    pub is_pend:     bool,
    pub pend_scale:  Option<u8>,
    pub pend_effect: String,

    pub link_arrows: Vec<monster::LinkArrow>,

    pub limit:   u8,
}

impl TryFrom<CardData> for MonsterCard
{
    type Error = ah::Error;

    fn try_from(data: CardData) -> Result<Self, Self::Error>
    {
        Ok(Self {
            id:      data.id,
            name:    data.name,
            creator: data.username,

            is_effect: data.is_effect > 0,
            effect:    data.effect,

            kind:         monster::Kind::try_from(data.monster_color)?,
            level:        data.level,
            attribute:    monster::Attribute::try_from(data.attribute)?,
            monster_type: monster::Type::try_from(data.r#type)?,
            ability:      monster::Ability::try_from_many(data.ability)?,
            flip:         data.flip > 0,
            atk:          monster::try_parse_atk_def(data.atk)?,
            def:          monster::try_parse_atk_def(data.def)?,
            is_pend:      data.pendulum > 0,
            pend_scale:   Some(data.scale),  // TODO:FIXME: How does DB indicate no scale?
            pend_effect:  data.pendulum_effect,

            link_arrows:  monster::LinkArrow::try_from_many(data.arrows)?,

            limit:        data.tcg_limit,
        })
    }
}


pub use monster::*;

mod monster
{
    use anyhow as ah;


#[derive(serde::Serialize, Copy, Clone, PartialEq, Debug)]
pub enum Kind {
    NORMAL, EFFECT, RITUAL,
    FUSION, SYNCHRO, XYZ, LINK,
}

impl TryFrom<String> for Kind {
    type Error = ah::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Normal"  => Ok(Self::NORMAL),
            "Effect"  => Ok(Self::EFFECT),
            "Ritual"  => Ok(Self::RITUAL),
            "Fusion"  => Ok(Self::FUSION),
            "Synchro" => Ok(Self::SYNCHRO),
            "Xyz"     => Ok(Self::XYZ),
            "Link"    => Ok(Self::LINK),
            _ => Err(ah::anyhow!("Received invalid Monster Card Type: `{value}`")),
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::NORMAL  => "Normal",
            Self::EFFECT  => "",
            Self::RITUAL  => "Ritual",
            Self::FUSION  => "Fusion",
            Self::SYNCHRO => "Synchro",
            Self::XYZ     => "Xyz",
            Self::LINK    => "Link",
        })
    }
}


#[derive(serde::Serialize, Copy, Clone, PartialEq, Debug)]
pub enum Attribute {
    LIGHT, DARK, WATER, FIRE, EARTH, WIND, DIVINE,
}

impl TryFrom<String> for Attribute {
    type Error = ah::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "LIGHT" => Ok(Self::LIGHT),
            "DARK"  => Ok(Self::DARK),
            "WATER" => Ok(Self::DARK),
            "FIRE"  => Ok(Self::FIRE),
            "EARTH" => Ok(Self::EARTH),
            "WIND"  => Ok(Self::WIND),
            _ => Err(ah::anyhow!("Received invalid Monster Attribute: `{value}`")),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::LIGHT  => "LIGHT",
            Self::DARK   => "DARK",
            Self::WATER  => "WATER",
            Self::FIRE   => "FIRE",
            Self::EARTH  => "EARTH",
            Self::WIND   => "WIND",
            Self::DIVINE => "DIVINE",
        })
    }
}


#[derive(serde::Serialize, Copy, Clone, PartialEq, Debug)]
pub enum Type {
    AQUA, BEAST, BEAST_WARRIOR, CELESTIAL_WARRIOR, CYBERSE, CYBORG, DRAGON, DINOSAUR, DIVINE_BEAST, FAIRY, FIEND, FISH, GALAXY, INSECT, ILLUSION, MACHINE, MAGICAL_KNIGHT, OMEGA_PSYCHIC, PLANT, PSYCHIC, PYRO, REPTILE, ROCK, SEA_SERPENT, SPELLCASTER, THUNDER, WARRIOR, WINGED_BEAST, WYRM, ZOMBIE
}

impl TryFrom<String> for Type {
    type Error = ah::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "aqua"              => Ok(Self::AQUA),
            "beast"             => Ok(Self::BEAST),
            "beast-warrior"     => Ok(Self::BEAST_WARRIOR),
            "celestial warrior" => Ok(Self::CELESTIAL_WARRIOR),
            "cyberse"           => Ok(Self::CYBERSE),
            "cyborg"            => Ok(Self::CYBORG),
            "dragon"            => Ok(Self::DRAGON),
            "dinosaur"          => Ok(Self::DINOSAUR),
            "divine-beast"      => Ok(Self::DIVINE_BEAST),
            "fairy"             => Ok(Self::FAIRY),
            "fiend"             => Ok(Self::FIEND),
            "fish"              => Ok(Self::FISH),
            "galaxy"            => Ok(Self::GALAXY),
            "insect"            => Ok(Self::INSECT),
            "illusion"          => Ok(Self::ILLUSION),
            "machine"           => Ok(Self::MACHINE),
            "magical knight"    => Ok(Self::MAGICAL_KNIGHT),
            "omega psychic"     => Ok(Self::OMEGA_PSYCHIC),
            "plant"             => Ok(Self::PLANT),
            "psychic"           => Ok(Self::PSYCHIC),
            "pyro"              => Ok(Self::PYRO),
            "reptile"           => Ok(Self::REPTILE),
            "rock"              => Ok(Self::ROCK),
            "sea serpent"       => Ok(Self::SEA_SERPENT),
            "spellcaster"       => Ok(Self::SPELLCASTER),
            "thunder"           => Ok(Self::THUNDER),
            "warrior"           => Ok(Self::WARRIOR),
            "winged beast"      => Ok(Self::WINGED_BEAST),
            "wyrm"              => Ok(Self::WYRM),
            "zombie"            => Ok(Self::ZOMBIE),
            _ => Err(ah::anyhow!("Received invalid Monster Type: `{value}`")),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::AQUA              => "Aqua",
            Self::BEAST             => "Beast",
            Self::BEAST_WARRIOR     => "Beast-Warrior",
            Self::CELESTIAL_WARRIOR => "Celestial Warrior",
            Self::CYBERSE           => "Cyberse",
            Self::CYBORG            => "Cyborg",
            Self::DRAGON            => "Dragon",
            Self::DINOSAUR          => "Dinosaur",
            Self::DIVINE_BEAST      => "Divine-Beast",
            Self::FAIRY             => "Fairy",
            Self::FIEND             => "Fiend",
            Self::FISH              => "Fish",
            Self::GALAXY            => "Galaxy",
            Self::INSECT            => "Insect",
            Self::ILLUSION          => "Illusion",
            Self::MACHINE           => "Machine",
            Self::MAGICAL_KNIGHT    => "Magical Knight",
            Self::OMEGA_PSYCHIC     => "Omega Psychic",
            Self::PLANT             => "Plant",
            Self::PSYCHIC           => "Psychic",
            Self::PYRO              => "Pyro",
            Self::REPTILE           => "Reptile",
            Self::ROCK              => "Rock",
            Self::SEA_SERPENT       => "Sea Serpent",
            Self::SPELLCASTER       => "Spellcaster",
            Self::THUNDER           => "Thunder",
            Self::WARRIOR           => "Warrior",
            Self::WINGED_BEAST      => "Winged Beast",
            Self::WYRM              => "Wyrm",
            Self::ZOMBIE            => "Zombie",
        })
    }
}


#[derive(serde::Serialize, Copy, Clone, PartialEq, Debug)]
pub enum Ability {
    GEMINI, SPIRIT, TOON, TUNER, UNION,
}

impl Ability {
    pub fn try_from_many(value: String) -> ah::Result<Vec<Self>> {
        if value == "" { return Ok(vec![]); }
        
        value.split(r#" \/ "#)
            .map(|s| Self::try_from(s.to_string()))
            .collect()
    }
}

impl TryFrom<String> for Ability {
    type Error = ah::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Gemini" => Ok(Self::GEMINI),
            "Spirit" => Ok(Self::SPIRIT),
            "Toon"   => Ok(Self::TOON),
            "Tuner"  => Ok(Self::TUNER),
            "Union"  => Ok(Self::UNION),
            _ => Err(ah::anyhow!("Received invalid Monster Ability: `{value}`")),
        }
    }
}

impl std::fmt::Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::GEMINI => "Gemini",
            Self::SPIRIT => "Spirit",
            Self::TOON   => "Toon",
            Self::TUNER  => "Tuner",
            Self::UNION  => "Union",
        })
    }
}


pub fn try_parse_atk_def(value: String) -> ah::Result<Option<u16>>
{
    if value == "?" { Ok(None) } else { Ok(Some(value.parse()?)) }
}


#[derive(serde::Serialize, Copy, Clone, PartialEq, Debug)]
pub enum LinkArrow {
    UP, DOWN, LEFT, RIGHT,
    UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
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

impl LinkArrow {
    pub fn try_from_many(value: String) -> ah::Result<Vec<Self>> {
        if value == "" { return Ok(vec![]); }

        let out: Vec<Self> =
            value
            .chars()
            .zip(LINK_ARROWS)
            .filter(|(c, _)| *c == '1')
            .map(|(_, arrow)| arrow)
            .collect();

        if out.len() != 8 {
            Err(ah::anyhow!("Received unexpected number of Link Arrows: `{}`", out.len()))
        } else {
            Ok(out)
        }
    }
}

impl std::fmt::Display for LinkArrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::UP_LEFT    => "TL",
            Self::UP         => "T",
            Self::UP_RIGHT   => "TR",
            Self::RIGHT      => "R",
            Self::DOWN_RIGHT => "DR",
            Self::DOWN       => "D",
            Self::DOWN_LEFT  => "DL",
            Self::LEFT       => "L",
        })
    }
}


}
