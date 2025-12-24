use anyhow as ah;
use rust_xlsxwriter as xlsx;
use xlsx::worksheet as worksheet;

use duelingbook_extractor as dbxt;


// == CSV == //
pub fn decks_to_csv(_decks: Vec<dbxt::Deck>) -> String
{
    unimplemented!()
}

pub fn _card_to_csv(_card: dbxt::Card) -> String
{
    unimplemented!()
}


// == XLSX == //
pub enum ExcelValue {
    Text(String),
    Int(u32),
    Blank,
}

#[duplicate::duplicate_item(int; [u8]; [u16]; [u32])]
impl From<int> for ExcelValue
{
    fn from(value: int) -> Self {
        Self::Int(value as u32)
    }
}

impl worksheet::IntoExcelData for ExcelValue
{
    fn write(self,
        sheet: &mut worksheet::Worksheet,
        row: worksheet::RowNum,
        col: worksheet::ColNum,
    ) -> Result<&mut worksheet::Worksheet, xlsx::XlsxError>
    {
        match self {
            Self::Text(val) => sheet.write_string(row, col, val),
            Self::Int(val)  => sheet.write_number(row, col, val),
            Self::Blank     => Ok(sheet),
        }
    }

    fn write_with_format<'a>(self,
        sheet: &'a mut worksheet::Worksheet,
        row: worksheet::RowNum,
        col: worksheet::ColNum,
        format: &rust_xlsxwriter::Format,
    ) -> Result<&'a mut worksheet::Worksheet, xlsx::XlsxError>
    {
        match self {
            Self::Text(val) => sheet.write_string_with_format(row, col, val, format),
            Self::Int(val)  => sheet.write_number_with_format(row, col, val, format),
            Self::Blank     => Ok(sheet),
        }
    }
}


pub fn _decks_to_xlsx(_decks: Vec<dbxt::Deck>) -> ah::Result<String>
{
    unimplemented!()
}

pub fn card_to_xlsx_row(card: dbxt::Card) -> Vec<(&'static str, ExcelValue)>
{
    match card {
        dbxt::Card::Monster(data) => vec![
            (
                "ID",
                ExcelValue::from(data.id)
            ), (
                "Card Name",
                ExcelValue::Text(data.name)
            ), (
                "Kind / Property",
                ExcelValue::Text(
                    chain(vec![
                        data.kind.to_string().as_str(),
                        if data.is_pend { "Pendulum" } else { "" }
                    ])
                )
            ), (
                "Level / Rank / Link",
                ExcelValue::from(data.level)
            ), (
                "Type",
                ExcelValue::Text(data.monster_type.to_string())
            ), (
                "Ability",
                ExcelValue::Text(chain(data.ability))
            ), (
                "Attribute",
                ExcelValue::Text(data.attribute.to_string())
            ), (
                "ATK",
                match data.atk {
                    Some(val) => ExcelValue::from(val),
                    None      => ExcelValue::Blank,
                }
            ), (
                "DEF",
                match data.def {
                    Some(val) => ExcelValue::from(val),
                    None      => ExcelValue::Blank,
                }
            ), (
                "Pendulum Scale",
                match data.pend_scale {
                    Some(val) => ExcelValue::from(val),
                    None      => ExcelValue::Blank,
                }
            ), (
                "Link Arrows",
                ExcelValue::Text(chain(data.link_arrows))
            ), (
                "Limit",
                ExcelValue::from(data.limit)
            ), (
                "Text",
                ExcelValue::Text(data.effect)
            )
        ],
    }
}

fn chain(segments: impl IntoIterator<Item = impl ToString>) -> String
{
    segments
    .into_iter()
    .filter_map(|s| {
        let out = s.to_string();
        if out != "" { Some(out) } else { None }
    })
    .collect::<Vec<_>>()
    .join(" / ")
}


#[cfg(test)] mod tests
{
    use super::*;

    #[test] fn _chain_() {
        assert_eq!( chain(["1", "2"]), "1 / 2" );
        assert_eq!( chain(["1", ""]), "1" );
        assert_eq!( chain(["", "2"]), "2" );
        assert_eq!( chain(["", ""]), "" );
    }
}
