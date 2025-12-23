mod extractor;
use extractor as xt;

use duelingbook_extractor as db;


fn main() {
    println!(">> running DuelingBook extractor...");

    let out = run("https://www.duelingbook.com/deck?id=18239213");

    match out {
        Err(msg) => println!("!! {msg}"),
        _        => println!(".. done!"),
    };
}


fn run(deck_url: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let data: db::DeckData = xt::fetch::deck(deck_url)?;

    let deck = db::Deck::from(data);

    Ok(())
}
