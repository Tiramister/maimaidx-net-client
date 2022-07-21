mod crawler;
mod parser;

use anyhow::{ensure, Context, Result};
use const_format::{concatcp, formatcp};
use crawler::{get_request, post_request};
use parser::{select_first_element, select_some_element};
use reqwest::{Client, ClientBuilder};
use scraper::Html;
use std::env;

use crate::parser::select_all_elements;

const ROOT_URL: &'static str = "https://maimaidx.jp/maimai-mobile/";

async fn get_login_token(client: &Client) -> Result<String> {
    const URL: &'static str = ROOT_URL;
    let login_page = get_request(client, URL, None::<&()>).await?;

    let html_document = Html::parse_document(&login_page);
    let html_element = html_document.root_element();

    let token_element = select_first_element(&html_element, "input[name='token']")?;
    let token = token_element
        .value()
        .attr("value")
        .context("The token element has no attribute 'value'.")?;
    Ok(token.to_string())
}

async fn login(client: &Client, sega_id: &str, sega_password: &str, token: &str) -> Result<()> {
    const URL: &str = concatcp!(ROOT_URL, "submit/");
    let params = [
        ("segaId", sega_id),
        ("password", sega_password),
        ("save_cookie", "on"),
        ("token", token),
    ];
    let after_login_page = post_request(client, URL, Some(&params)).await?;

    let html_document = Html::parse_document(&after_login_page);
    let html_element = html_document.root_element();

    let title_element = select_first_element(&html_element, "title")?;
    let title = title_element
        .text()
        .next()
        .context("The element has no contents.")?;

    // Aime 画面であれば OK
    ensure!(
        title.contains("Aime"),
        format!("Response is not the Aime select page, but {}.", title)
    );

    Ok(())
}

async fn select_aime(client: &Client, idx: i32) -> Result<()> {
    const URL: &'static str = concatcp!(ROOT_URL, "aimeList/submit/");
    let params = [("idx", idx)];
    let after_aime_page = get_request(client, URL, Some(&params)).await?;

    let html_document = Html::parse_document(&after_aime_page);
    let html_element = html_document.root_element();

    let title_element = select_first_element(&html_element, "title")?;
    let title = title_element
        .text()
        .next()
        .context("The element has no contents.")?;

    // ホーム画面であれば OK
    ensure!(
        title.contains("ホーム"),
        format!("Response is not the home page, but {}.", title)
    );

    Ok(())
}

async fn get_record_page(client: &Client, difficulty: i32) -> Result<()> {
    const URL: &'static str = concatcp!(ROOT_URL, "record/musicGenre/search/");
    let params = [("genre", 99), ("diff", difficulty)];
    let record_page = get_request(client, URL, Some(&params)).await?;

    let html_document = Html::parse_document(&record_page);
    let html_element = html_document.root_element();

    let title_element = select_first_element(&html_element, "title")?;
    let title = title_element
        .text()
        .next()
        .context("The element has no contents.")?;

    // レコード画面であれば OK
    ensure!(
        title.contains("楽曲スコア"),
        format!("Response is not the home page, but {}.", title)
    );

    const ACTION_URL: &'static str = concatcp!(ROOT_URL, "record/musicDetail/");
    const RECORD_SELECTOR: &'static str = formatcp!("form[action='{ACTION_URL}']");

    for record_element in select_all_elements(&html_element, RECORD_SELECTOR)? {
        // スコアがない=未プレイ
        if let Some(score_element) = select_some_element(&record_element, "div.music_score_block")?
        {
            let score = score_element
                .text()
                .next()
                .context("The score element has no contents.")?;

            let name = select_first_element(&record_element, "div.music_name_block")?
                .text()
                .next()
                .context("The name element has no contents.")?;

            println!("{name}: {score}");
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ClientBuilder::new().cookie_store(true).build().unwrap();

    let token = get_login_token(&client).await?;

    let sega_id = env::var("SEGA_ID").context("The environment variable SEGA_ID is not set.")?;
    let sega_password =
        env::var("SEGA_PASSWORD").context("The environment variable SEGA_PASSWORD is not set.")?;

    login(&client, &sega_id, &sega_password, &token)
        .await
        .context("Failed to login.")?;

    select_aime(&client, 0)
        .await
        .context("Failed to select aime.")?;

    get_record_page(&client, 3).await?;

    Ok(())
}
