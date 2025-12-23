#[derive(std::fmt::Debug)]
pub struct Options
{
    pub urls: Vec<String>,

    pub browser_wait: u64,

    pub export_path: Option<std::path::PathBuf>,
}


impl Default for Options
{
    fn default() -> Self
    {
        Self {
            urls:
                vec![
                    11724812,  // Essence
                    11423800,  // Vengeance
                    // 11882083,  // Vapurion
                    // 12053993,  // Darquessence
                    // 12381789,  // Rubix
                    // 18239213,  // Tritale
                ]
                .into_iter()
                .map(|id| format!("https://duelingbook.com/deck?id={id}"))
                .collect(),
            
            browser_wait: 2500,

            export_path: std::env::current_dir().ok(),
        }
    }
}
