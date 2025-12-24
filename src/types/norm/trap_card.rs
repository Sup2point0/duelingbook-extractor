use anyhow as ah;

use crate::types::db::CardData;


#[derive(serde::Serialize, PartialEq, Clone, Debug)]
pub struct TrapCard
{
    pub id:      u32,
    pub name:    String,
    pub creator: String,

    pub effect:   String,
    pub property: TrapProperty,
}

impl TryFrom<CardData> for TrapCard
{
    type Error = ah::Error;

    fn try_from(data: CardData) -> Result<Self, Self::Error>
    {
        Ok(Self {
            id:      data.id,
            name:    data.name,
            creator: data.username,

            effect:   data.effect,
            property: TrapProperty::try_from(data.r#type)?,
        })
    }
}


#[derive(serde::Serialize, Copy, Clone, PartialEq, Debug)]
pub enum TrapProperty {
    NORMAL, CONTINUOUS, COUNTER,
}

impl TryFrom<String> for TrapProperty {
    type Error = ah::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Normal"     => Ok(Self::NORMAL),
            "Continuous" => Ok(Self::CONTINUOUS),
            "Counter"    => Ok(Self::COUNTER),
            _ => Err(ah::anyhow!("Received invalid Trap TrapProperty: `{value}`")),
        }
    }
}

impl std::fmt::Display for TrapProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::NORMAL     => "",
            Self::CONTINUOUS => "Continuous",
            Self::COUNTER    => "Counter",
        })
    }
}
