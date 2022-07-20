use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

async fn get_login_token(client: &Client) -> Result<String> {
    let login_page = client
        .get("https://maimaidx.jp/maimai-mobile/")
        .send()
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
    let client = Client::new();
    let token = get_login_token(&client).await?;
    println!("{token}");
    Ok(())
}
