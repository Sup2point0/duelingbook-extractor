use clap::Parser;

mod cli;
pub use cli::*;

mod extractor;


fn main()
{
    println!(">> running DuelingBook extractor...");

    let status = run();

    match status {
        Err(msg) => println!("!! {msg}"),
        _        => println!(">> done!"),
    };
}


fn run() -> anyhow::Result<()>
{
    let cli = cli::Cli::parse();
    let mut exec = Executive::init(cli)?;

    exec.run()?;

    Ok(())
}
