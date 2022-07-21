use std::time::Duration;

use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use tokio::time::sleep;

/** 連続リクエスト回避 */
async fn sleep_shortly() {
    sleep(Duration::from_secs(1)).await;
}

pub async fn get_request<T>(client: &Client, url: &str, params_opt: Option<&T>) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let request_builder = if let Some(params) = params_opt {
        client.get(url).query(params)
    } else {
        client.get(url)
    };
    let response = request_builder.send().await?.text().await?;

    sleep_shortly().await;

    Ok(response)
}

pub async fn post_request<T>(client: &Client, url: &str, params_opt: Option<&T>) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let request_builder = if let Some(params) = params_opt {
        client.post(url).form(params)
    } else {
        client.post(url)
    };
    let response = request_builder.send().await?.text().await?;

    sleep_shortly().await;

    Ok(response)
}
