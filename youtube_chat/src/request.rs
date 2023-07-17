use serde::Serialize;

use crate::{
    item::ChatItem,
    parser::{get_options_from_live_page, parse_chat_data},
    youtube_types::{GetLiveChatBody, GetLiveChatResponse},
};

#[derive(Clone, Serialize, Debug)]
pub struct RequestOptions {
    pub api_key: String,
    pub client_version: String,
    pub continuation: String,
}

pub async fn fetch_chat<'a>(
    options: RequestOptions,
) -> Result<(Vec<ChatItem>, String), anyhow::Error> {
    let url = format!(
        "https://www.youtube.com/youtubei/v1/live_chat/get_live_chat?key={}",
        options.api_key
    );
    let body = GetLiveChatBody::new(
        options.continuation,
        options.client_version,
        "WEB".to_string(),
    );
    let client = reqwest::Client::new();
    let response = client.post(url).json(&body).send().await?;
    let text = response.text().await.unwrap();
    let json: GetLiveChatResponse = serde_json::from_str(&text)?;
    Ok(parse_chat_data(json))
}

pub async fn fetch_live_page(url: String) -> Result<(RequestOptions, String), anyhow::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?.text().await?;
    get_options_from_live_page(response)
}
