use anyhow::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let login_page = reqwest::get("https://maimaidx.jp/maimai-mobile/")
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&login_page);
    let token_selector = Selector::parse("input[name='token']").unwrap();
    let token_element = document.select(&token_selector).next().unwrap();
    let token = token_element.value().attr("value").unwrap();
    println!("{token}");
    Ok(())
}
