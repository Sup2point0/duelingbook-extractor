use playwright_rs as play;


#[tokio::main]
pub async fn visit(url: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let playwright = play::Playwright::launch().await?;
    let browser = playwright.chromium().launch().await?;
    let page = browser.new_page().await?;

    page.on_console(|msg| {
        println!("{}", msg.text());
    });
    
    page.goto(url, None).await;

    Ok(())
}
