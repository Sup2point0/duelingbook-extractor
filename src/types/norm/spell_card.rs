use anyhow as ah;

use crate::types::db::CardData;


#[derive(serde::Serialize, PartialEq, Clone, Debug)]
pub struct SpellCard
{
    pub id:      u32,
    pub name:    String,
    pub creator: String,

    pub effect:   String,
    pub property: SpellProperty,
}

impl TryFrom<CardData> for SpellCard
{
    type Error = ah::Error;

    fn try_from(data: CardData) -> Result<Self, Self::Error>
    {
        Ok(Self {
            id:      data.id,
            name:    data.name,
            creator: data.username,

            effect:   data.effect,
            property: SpellProperty::try_from(data.r#type)?,
        })
    }
}


#[derive(serde::Serialize, Copy, Clone, PartialEq, Debug)]
pub enum SpellProperty {
    NORMAL, QUICK_PLAY, CONTINUOUS,
    FIELD, EQUIP, RITUAL,
}

impl TryFrom<String> for SpellProperty {
    type Error = ah::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Normal"     => Ok(Self::NORMAL),
            "Quick-Play" => Ok(Self::QUICK_PLAY),
            "Continuous" => Ok(Self::CONTINUOUS),
            "Field"      => Ok(Self::FIELD),
            "Equip"      => Ok(Self::EQUIP),
            "Ritual"     => Ok(Self::RITUAL),
            _ => Err(ah::anyhow!("Received invalid Spell SpellProperty: `{value}`")),
        }
    }
}

impl std::fmt::Display for SpellProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::NORMAL     => "",
            Self::QUICK_PLAY => "Quick",
            Self::CONTINUOUS => "Continuous",
            Self::FIELD      => "Field",
            Self::EQUIP      => "Equip",
            Self::RITUAL     => "Ritual",
        })
    }
}
