use anyhow as ah;
use clap;


const HELP_IDS:  &'static str = "List of Deck IDs";
const HELP_URLS: &'static str = "List of Deck URLs";
const HELP_WAIT: &'static str = "How long (in ms) the browser will wait for the page to load";


#[derive(clap::Parser, std::fmt::Debug)]
#[command(version, about, long_about = None)]
pub struct Cli
{
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(clap::Subcommand, Clone, std::fmt::Debug)]
pub enum Mode
{
    #[command(about = "Debug command for testing")]
    DEBUG {},

    /* Don't mind the alignment, I have supcode DualShift in VSCode which changes it ;P */
    // TODO: Refactor duplication with a macro

    #[command(about = "Export decks data to a .json file")]
    JSON {
        #[arg(short = 'i', long = "ids", help = HELP_IDS)]   ids: Option<Vec<u32>>,
        #[arg(short = 'u', long = "urls", help = HELP_URLS)] urls: Option<Vec<String>>,

        #[arg(short = 'o', long = "export", value_parser = Cli::from_root)]
        export_path: Option<std::path::PathBuf>,

        #[arg(long = "browser-wait", help = HELP_WAIT)]     browser_wait: Option<u64>,
    },
    #[command(about = "Export decks data to a .csv file")]
    CSV {
        #[arg(short = 'i', long = "ids", help = HELP_IDS)]   ids: Option<Vec<u32>>,
        #[arg(short = 'u', long = "urls", help = HELP_URLS)] urls: Option<Vec<String>>,

        #[arg(short = 'o', long = "export", value_parser = Cli::from_root)]
        export_path: Option<std::path::PathBuf>,

        #[arg(long = "browser-wait", help = HELP_WAIT)]     browser_wait: Option<u64>,
    },
    #[command(about = "Export decks data to a .xlsx file")]
    XLSX {
        #[arg(short = 'i', long = "ids", help = HELP_IDS)]   ids: Option<Vec<u32>>,
        #[arg(short = 'u', long = "urls", help = HELP_URLS)] urls: Option<Vec<String>>,

        #[arg(short = 'o', long = "export", value_parser = Cli::from_root)]
        export_path: Option<std::path::PathBuf>,

        #[arg(long = "browser-wait", help = HELP_WAIT)]     browser_wait: Option<u64>,
    },
}

impl Cli
{
    pub fn from_root(s: &str) -> ah::Result<std::path::PathBuf>
    {
        Ok(std::env::current_dir()?.join(s))
    }
}
