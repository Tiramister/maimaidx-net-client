use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::env;

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

    let sega_id = env::var("SEGA_ID").context("The environment variable SEGA_ID is not set.")?;
    let sega_password =
        env::var("SEGA_PASSWORD").context("The environment variable SEGA_PASSWORD is not set.")?;
    println!("{sega_id} {sega_password}");

    Ok(())
}
