use crate::cli;
use crate::extractor as xt;

use duelingbook_extractor as dbxt;


pub struct Executive
{
    cli: cli::Cli,
}

impl Executive
{
    pub fn new(cli: cli::Cli) -> Self
    {
        Self { cli }
    }

    pub fn run(&self) -> anyhow::Result<()>
    {
        println!(".. Fetching decks data...");
        let decks = xt::fetch::decks(&self.cli.options)?;

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

    fn exec_json(&self, decks: Vec<dbxt::Deck>) -> anyhow::Result<()>
    {
        println!(".. Exporting data to {:?}....", self.cli.options.export_path);
        unimplemented!()
    }

    fn exec_csv(&self, decks: Vec<dbxt::Deck>) -> anyhow::Result<()>
    {
        println!(".. Exporting data to {:?}...", self.cli.options.export_path);
        unimplemented!()
    }

    fn exec_xlsx(&self, decks: Vec<dbxt::Deck>) -> anyhow::Result<()>
    {
        println!(".. Exporting data to {:?}....", self.cli.options.export_path);
        unimplemented!()
    }
}
