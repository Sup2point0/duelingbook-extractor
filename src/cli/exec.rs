use anyhow as ah;
use rust_xlsxwriter as xlsx;
use serde_json as json;

use crate::cli;
use crate::extractor as xt;

use duelingbook_extractor as dbxt;


pub struct Executive
{
    cli: cli::Cli,
    options: cli::Options,
}

impl Executive
{
    pub fn init(cli: cli::Cli) -> ah::Result<Self>
    {
        let options = cli::Options::from_cli(cli.mode.clone())?;
        Ok(Self { cli, options })
    }
}

impl Executive
{
    pub fn run(&mut self) -> ah::Result<()>
    {
        println!(".. Fetching decks data...");
        let decks = xt::fetch::decks(&self.options)?;

        match self.cli.mode {
            cli::Mode::JSON{..} => self.exec_json(decks)?,
            cli::Mode::CSV {..} => self.exec_csv(decks)?,
            cli::Mode::XLSX{..} => self.exec_xlsx(decks)?,
            _ => Self::debug(decks),
        }

        Ok(())
    }

    fn debug(decks: Vec<dbxt::Deck>) -> ()
    {
        println!(".. Showing debug output...");
        for deck in decks {
            println!("deck = {}", deck);
        }
    }

    fn exec_json(&mut self, decks: Vec<dbxt::Deck>) -> ah::Result<()>
    {
        let export = json::to_string(&decks)?;
        self.save(export)?;

        Ok(())
    }

    fn exec_csv(&self, decks: Vec<dbxt::Deck>) -> ah::Result<()>
    {
        let export = xt::export::decks_to_csv(decks);
        self.save(export)?;

        Ok(())
    }

    fn exec_xlsx(&self, decks: Vec<dbxt::Deck>) -> ah::Result<()>
    {
        let mut book = xlsx::Workbook::new();
        let sheet = book.add_worksheet();

        let mut row = 0;

        for deck in decks {
            for card in deck.main {
                let data = xt::export::card_to_xlsx_row(card);
                let data = data.into_iter().map(|(_col, val)| val);
                sheet.write_row(row, 0, data)?;
            }

            row += 1;
        }

        book.save(&self.options.export_path)?;

        Ok(())
    }
}

impl Executive
{
    fn save(&self, contents: String) -> ah::Result<()>
    {
        let path = self.options.export_path.clone();
        println!(".. Exporting data to {:?}....", path);

        std::fs::write(path, contents)?;

        Ok(())
    }
}
