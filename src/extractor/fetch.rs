use chromiumoxide::{
    self as cr2o3,
    cdp::js_protocol::runtime::{ConsoleApiCalledType, EventConsoleApiCalled}
};
use tokio as tk;
use futures::StreamExt;
use serde_json as json;


const DECK_RESPONSE_START: &str = r#"{"action":"#;
const DECK_RESPONSE_START_SUCCESS: &str = r#"{"action":"Success","#;


#[tk::main]
pub async fn deck(url: &str) -> Result<json::Value, Box<dyn std::error::Error>>
{
    let (mut browser, mut handler) = cr2o3::Browser::launch(
        cr2o3::BrowserConfig::builder().with_head().build()?
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

    tk::time::sleep(tk::time::Duration::from_secs(3)).await;

    browser.close().await?;
    console_logs.await?;
    handle.await?;

    let mut deck = None;

    while let Some(content) = rx.recv().await {
        if let Some(data) = try_parse_deck_json(&content)? {
            println!("data = {:?}", data.to_string().truncate(20));
            deck = Some(data);
            break;
        }
    }

    deck.ok_or(Err("failed to find deck content")?)
}

pub fn extract_log_text(event: &EventConsoleApiCalled) -> Option<json::Value>
{
    match event.r#type {
        ConsoleApiCalledType::Log => event.args[0].value.clone(),
        _ => None,
    }
}

pub fn try_parse_deck_json(data: &json::Value) -> anyhow::Result<Option<json::Value>>
{
    match data {
        json::Value::String(text) => {
            if !text.starts_with(DECK_RESPONSE_START) {
                return Ok(None);
            }

            if !text.starts_with(DECK_RESPONSE_START_SUCCESS) {
                return Err(anyhow::anyhow!("Webpage failed to retrieve deck"));
            }

            let out = json::from_str(text)?;
            Ok(Some(out))
        },
        _ => Ok(None)
    }
}
