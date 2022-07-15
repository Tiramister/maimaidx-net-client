use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let login_page = client
        .get("https://maimaidx.jp/maimai-mobile/")
        .send()
        .await?
        .text()
        .await?;
    println!("{login_page}");
    Ok(())
}
