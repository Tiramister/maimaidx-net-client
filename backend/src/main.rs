use anyhow::Result;
use scraper::{Html, Selector};

async fn get_login_token() -> Result<String> {
    let login_page = reqwest::get("https://maimaidx.jp/maimai-mobile/")
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&login_page);
    let token_selector = Selector::parse("input[name='token']").unwrap();
    let token_element = document.select(&token_selector).next().unwrap();
    let token = token_element.value().attr("value").unwrap();
    Ok(token.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = get_login_token().await?;
    println!("{token}");
    Ok(())
}
