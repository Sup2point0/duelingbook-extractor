use anyhow as ah;

use crate::cli;


#[derive(std::fmt::Debug)]
pub struct Options
{
    pub urls: Vec<String>,
    pub export_path: std::path::PathBuf,

    pub browser_wait: u64,
}

impl Options
{
    pub fn init(
        file_extension: &str,
        ids: Option<Vec<u32>>,
        urls: Option<Vec<String>>,
        export_path: Option<std::path::PathBuf>,
        browser_wait: Option<u64>,
    ) -> ah::Result<Self>
    {
        Ok(Self {
            urls: ids.map(Self::urls_from_ids)
                    .unwrap_or(urls
                    .unwrap_or(defaults::urls())
                ),
            export_path:  export_path .unwrap_or(defaults::export_path()?).with_extension(file_extension),
            browser_wait: browser_wait.unwrap_or(defaults::browser_wait()),
        })
    }

    pub fn from_cli(mode: cli::Mode) -> ah::Result<Self> {
        match mode {
            cli::Mode::JSON { ids, urls, export_path, browser_wait } => Self::init(".json", ids, urls, export_path, browser_wait),
            cli::Mode::CSV  { ids, urls, export_path, browser_wait } => Self::init(".csv",  ids, urls, export_path, browser_wait),
            cli::Mode::XLSX { ids, urls, export_path, browser_wait } => Self::init(".xlsx", ids, urls, export_path, browser_wait),
            _ => Self::init("", None, None, None, None),
        }
    }
}

impl Options
{
    fn urls_from_ids(ids: Vec<u32>) -> Vec<String>
    {
        ids.into_iter()
            .map(|id| format!("https://duelingbook.com/deck?id={id}"))
            .collect()
    }
}


mod defaults
{
    use anyhow as ah;

    use super::Options;

    pub fn urls() -> Vec<String> { Options::urls_from_ids(
        vec![
            11724812,  // Essence
            11423800,  // Vengeance
            // 11882083,  // Vapurion
            // 12053993,  // Darquessence
            // 12381789,  // Rubix
            // 18239213,  // Tritale
        ]
    ) }

    pub fn export_path() -> ah::Result<std::path::PathBuf> {
        Ok(std::env::current_dir()?.join("duelingbook-extractor-export"))
    }
    
    pub fn browser_wait() -> u64 { 2500 }
}
