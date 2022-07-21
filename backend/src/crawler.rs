use anyhow::Result;
use reqwest::Client;
use serde::Serialize;

pub async fn get_request<T>(client: &Client, url: &str, params_opt: Option<&T>) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let request_builder = if let Some(params) = params_opt {
        client.get(url).query(params)
    } else {
        client.get(url)
    };
    Ok(request_builder.send().await?.text().await?)
}

