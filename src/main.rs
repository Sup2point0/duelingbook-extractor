use tokio as tk;

mod config;
mod extractor; use extractor as xt;

use duelingbook_extractor as db;


pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;


fn main() {
    println!(">> running DuelingBook extractor...");

    let out = run(config::deck_urls());

    match out {
        Err(msg) => println!("!! {msg}"),
        _        => println!(".. done!"),
    };
}


#[tk::main]
async fn run(deck_urls: Vec<&str>) -> AnyResult<()>
{
    let tasks = deck_urls.into_iter().enumerate().map(|(i, url)| async move {
        println!(">> exporting deck #{}", i+1);
        let data: db::DeckData = xt::fetch::deck(url).await?;
        println!("-- received data from browser");
        let deck = db::Deck::from(data);
        println!(">> finished exporting deck #{}", i+1);

        Ok(deck) as Result<db::Deck, Box<dyn std::error::Error>>
    });

    // let go = futures::future::join_all(tasks);
    // let results = go.await;
    
    let mut results = vec![];
    for task in tasks {
        results.push(task.await);
    }

    let decks = results.into_iter().collect::<AnyResult<Vec<db::Deck>>>()?;
    // let decks = results.into_iter().filter_map(|r| r.ok());

    println!(">> showing decks...");
    for deck in decks {
        println!("deck = {}", deck);
    }

    Ok(())
}
