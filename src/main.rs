mod extractor;


fn main() {
    println!(">> running DuelingBook extractor...");

    let out = run();

    match out {
        Err(msg) => println!("{msg}"),
        _        => (),
    };
}


fn run() -> Result<(), Box<dyn std::error::Error>>
{
    let deck = extractor::fetch::deck("https://www.duelingbook.com/deck?id=18239213")?;

    println!("got deck data");

    for card in deck.main {
        println!("{card:?}");
    }

    Ok(())
}
