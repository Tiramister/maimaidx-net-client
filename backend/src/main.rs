use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder};
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

async fn login(client: &Client, sega_id: &str, sega_password: &str, token: &str) {
    let params = [
        ("segaId", sega_id),
        ("password", sega_password),
        ("save_cookie", "on"),
        ("token", token),
    ];
    let response = client
        .post("https://maimaidx.jp/maimai-mobile/submit/")
        .form(&params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{response}")
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ClientBuilder::new().cookie_store(true).build().unwrap();

    let token = get_login_token(&client).await?;

    let sega_id = env::var("SEGA_ID").context("The environment variable SEGA_ID is not set.")?;
    let sega_password =
        env::var("SEGA_PASSWORD").context("The environment variable SEGA_PASSWORD is not set.")?;

    login(&client, &sega_id, &sega_password, &token).await;

    Ok(())
}
