mod crawler;

use anyhow::{ensure, Context, Result};
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

    // トークンを抽出
    let document = Html::parse_document(&login_page);
    let token_selector = Selector::parse("input[name='token']").unwrap();
    let token_element = document
        .select(&token_selector)
        .next()
        .context("There is no input element with its name 'token'.")?;
    let token = token_element
        .value()
        .attr("value")
        .context("The token element has no attribute 'value'.")?;

    Ok(token.to_string())
}

async fn login(client: &Client, sega_id: &str, sega_password: &str, token: &str) -> Result<()> {
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
        .await?
        .text()
        .await?;

    // タイトルを抽出
    let document = Html::parse_document(&response);
    let title_selector = Selector::parse("title").unwrap();
    let title_element = document
        .select(&title_selector)
        .next()
        .context("There is no title element.")?;
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
    let params = [("idx", idx)];
    let response = client
        .get("https://maimaidx.jp/maimai-mobile/aimeList/submit/")
        .query(&params)
        .send()
        .await?
        .text()
        .await?;

    // タイトルを抽出
    let document = Html::parse_document(&response);
    let title_selector = Selector::parse("title").unwrap();
    let title_element = document
        .select(&title_selector)
        .next()
        .context("There is no title element.")?;
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
    let params = [("genre", 99), ("diff", difficulty)];
    let response = client
        .get("https://maimaidx.jp/maimai-mobile/record/musicGenre/search/")
        .query(&params)
        .send()
        .await?
        .text()
        .await?;

    // タイトルを抽出
    let document = Html::parse_document(&response);
    let title_selector = Selector::parse("title").unwrap();
    let title_element = document
        .select(&title_selector)
        .next()
        .context("There is no title element.")?;
    let title = title_element
        .text()
        .next()
        .context("The element has no contents.")?;

    // レコード画面であれば OK
    ensure!(
        title.contains("楽曲スコア"),
        format!("Response is not the home page, but {}.", title)
    );

    let record_selector =
        Selector::parse("form[action='https://maimaidx.jp/maimai-mobile/record/musicDetail/']")
            .unwrap();
    let name_selector = Selector::parse("div.music_name_block").unwrap();
    let score_selector = Selector::parse("div.music_score_block").unwrap();

    for record_element in document.select(&record_selector) {
        // スコアがない=未プレイ
        if let Some(score_element) = record_element.select(&score_selector).next() {
            let score = score_element
                .text()
                .next()
                .context("The score element has no contents.")?;

            let name = record_element
                .select(&name_selector)
                .next()
                .context("There is no music name.")?
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
