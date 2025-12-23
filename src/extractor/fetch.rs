use anyhow as ah;
use chromiumoxide::{
    self as cr2o3,
    browser::HeadlessMode,
    cdp::js_protocol::runtime::{ConsoleApiCalledType, EventConsoleApiCalled}
};
use futures::StreamExt;
use serde_json as json;
use tokio as tk;

use crate::*;
use duelingbook_extractor as dbxt;


const DECK_RESPONSE_START: &str = r#"{"action":"#;
const DECK_RESPONSE_START_SUCCESS: &str = r#"{"action":"Success","#;


#[tk::main]
pub async fn decks(options@Options{ urls, .. }: &Options) -> ah::Result<Vec<dbxt::Deck>>
{
    let tasks = urls.into_iter().enumerate().map(|(i, url)| async move {
        println!(".. exporting deck #{}", i+1);
        let data: dbxt::DeckData = deck(url, options).await?;
        println!("-- received data from browser");
        let deck = dbxt::Deck::from(data);
        println!(".. finished exporting deck #{}", i+1);

        Ok(deck) as ah::Result<dbxt::Deck>
    });

    // let go = futures::future::join_all(tasks);
    // let results = go.await;
    
    let mut results = vec![];
    for task in tasks {
        results.push(task.await);
    }

    let decks = results.into_iter().collect::<ah::Result<Vec<dbxt::Deck>>>();
    // let decks = results.into_iter().filter_map(|r| r.ok());

    decks
}


pub async fn deck(url: &str, options: &Options) -> ah::Result<dbxt::DeckData>
{
    let (mut browser, mut handler) = cr2o3::Browser::launch(
        cr2o3::BrowserConfig::builder().with_head().build().map_err(|e| ah::anyhow!(e))?
        // cr2o3::BrowserConfig::builder().headless_mode(HeadlessMode::New).build().map_err(|e| ah::anyhow!(e))?
        // FIXME: Not sure why headless bugs out? Could be an issue on DuelingBook's end...
    ).await?;

    let handle = tk::spawn(async move {
        while let Some(_) = handler.next().await {}
    });

    let page = browser.new_page(url).await?;
    let mut listener = page.event_listener::<EventConsoleApiCalled>().await?;

    let (tx, mut rx) = tk::sync::mpsc::unbounded_channel();

    let console_logs = tk::spawn(async move {
        while let Some(event) = listener.next().await {
            if let Some(content) = extract_log_text(&*event) {
                let _ = tx.send(content);
            }
        }
    });

    tk::time::sleep(tk::time::Duration::from_millis(options.browser_wait)).await;

    browser.close().await?;
    console_logs.await?;
    handle.await?;

    let mut deck = None;

    while let Some(content) = rx.recv().await {
        let tried = try_parse_deck_json(&content);
        match tried {
            Ok(Some(data)) => {
                deck = Some(Ok(data));
                break;
            },
            Err(msg) => {
                deck = Some(Err(msg));
                break;
            },
            _ => (),
        }
    }

    match deck {
        Some(data) => data,
        None       => Err(ah::anyhow!("failed to find deck content"))
    }
}

pub fn extract_log_text(event: &EventConsoleApiCalled) -> Option<json::Value>
{
    match event.r#type {
        ConsoleApiCalledType::Log => event.args[0].value.clone(),
        _ => None,
    }
}

pub fn try_parse_deck_json(data: &json::Value) -> ah::Result<Option<dbxt::DeckData>>
{
    match data {
        json::Value::String(text) => {
            if !text.starts_with(DECK_RESPONSE_START) {
                return Ok(None);
            }

            if !text.starts_with(DECK_RESPONSE_START_SUCCESS) {
                return Err(ah::anyhow!(
                    "webpage failed to retrieve deck, gave response: `{}...`",
                    text.chars().take(50).collect::<String>()
                ));
            }

            let out = json::from_str(text)?;
            Ok(Some(out))
        },
        _ => Ok(None)
    }
}
