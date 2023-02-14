use crate::{
    item::ChatItem,
    parser::parse_chat_data,
    youtube_types::{GetLiveChatBody, GetLiveChatResponse},
};

pub struct ReqestOptions<'a> {
    pub api_key: &'a str,
    pub client_version: &'a str,
    pub continuation: &'a str,
}

pub async fn fetch_chat<'a>(
    options: ReqestOptions<'a>,
) -> Result<(Vec<ChatItem>, String), anyhow::Error> {
    let url = format!(
        "https://www.youtube.com/youtubei/v1/live_chat/get_live_chat?key={}",
        options.api_key
    );
    let body = GetLiveChatBody::new(
        options.continuation.to_string(),
        options.client_version.to_string(),
        "WEB".to_string(),
    );
    let client = reqwest::Client::new();
    let response = client.post(url).json(&body).send().await?;
    let json: GetLiveChatResponse = response.json().await?;
    Ok(parse_chat_data(json))
}
