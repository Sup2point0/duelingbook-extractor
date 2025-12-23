use crate::Options;


#[derive(clap::Parser, std::fmt::Debug)]
#[command(version, about, long_about = None)]
pub struct Cli
{
    #[command(subcommand)]
    pub mode: Mode,

    #[clap(skip)]
    pub options: Options,
}

#[derive(clap::Subcommand, std::fmt::Debug)]
pub enum Mode
{
    DEBUG {},
    
    JSON {},

    CSV {},

    XLSX {},
}
