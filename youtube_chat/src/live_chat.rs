use anyhow::anyhow;
use url::Url;

use crate::{
    item::ChatItem,
    request::{fetch_chat, fetch_live_page, RequestOptions},
};

/// SF, ENF, CF, ERF is `()` or `T: Fn()`
pub struct LiveChatClient<SF, ENF, CF, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    live_url: String,
    on_start: SF,
    on_end: ENF,
    on_chat: CF,
    on_error: ERF,
    options: Option<RequestOptions>,
}

impl<SF, ENF, CF, ERF> LiveChatClient<SF, ENF, CF, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    pub async fn execute(&mut self) {
        if let Some(mut options) = self.options.clone() {
            let result: Result<(), anyhow::Error> = async {
                let (chat_items, continuation) = fetch_chat(options.clone()).await?;
                for chat_item in chat_items {
                    self.invoke_on_chat(chat_item);
                }
                options.continuation = continuation;
                self.options = Some(options);
                Ok(())
            }
            .await;
            if let Err(err) = result {
                self.invoke_on_error(err);
            }
        } else {
            self.invoke_on_error(anyhow!(
                "This client is not ready for execute, just call `start`"
            ));
        }
    }

    pub async fn start(&mut self) -> Result<(), anyhow::Error> {
        let (options, live_id) = fetch_live_page(self.live_url.clone()).await?;
        self.options = Some(options);
        self.invoke_on_start(live_id);
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), anyhow::Error> {
        self.options = None;
        self.invoke_on_end();
        Ok(())
    }
}

pub struct Empty;
pub trait InvokeOnStart {
    fn invoke_on_start(&self, _live_id: String) {}
}
pub trait InvokeOnEnd {
    fn invoke_on_end(&self) {}
}
pub trait InvokeOnChat {
    fn invoke_on_chat(&self, _chat_item: ChatItem) {}
}
pub trait InvokeOnError {
    fn invoke_on_error(&self, _error: anyhow::Error) {}
}
impl InvokeOnStart for Empty {}
impl InvokeOnEnd for Empty {}
impl InvokeOnChat for Empty {}
impl InvokeOnError for Empty {}
impl<T> InvokeOnStart for T
where
    T: Fn(String),
{
    fn invoke_on_start(&self, live_id: String) {
        (self)(live_id)
    }
}
impl<T> InvokeOnEnd for T
where
    T: Fn(),
{
    fn invoke_on_end(&self) {
        (self)()
    }
}
impl<T> InvokeOnChat for T
where
    T: Fn(ChatItem),
{
    fn invoke_on_chat(&self, chat_item: ChatItem) {
        (self)(chat_item)
    }
}
impl<T> InvokeOnError for T
where
    T: Fn(anyhow::Error),
{
    fn invoke_on_error(&self, error: anyhow::Error) {
        (self)(error)
    }
}
impl<SF, ENF, CF, ERF> InvokeOnStart for LiveChatClient<SF, ENF, CF, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    fn invoke_on_start(&self, live_id: String) {
        self.on_start.invoke_on_start(live_id)
    }
}
impl<SF, ENF, CF, ERF> InvokeOnEnd for LiveChatClient<SF, ENF, CF, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    fn invoke_on_end(&self) {
        self.on_end.invoke_on_end()
    }
}
impl<SF, ENF, CF, ERF> InvokeOnChat for LiveChatClient<SF, ENF, CF, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    fn invoke_on_chat(&self, chat_item: ChatItem) {
        self.on_chat.invoke_on_chat(chat_item)
    }
}
impl<SF, ENF, CF, ERF> InvokeOnError for LiveChatClient<SF, ENF, CF, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    fn invoke_on_error(&self, error: anyhow::Error) {
        self.on_error.invoke_on_error(error)
    }
}

pub struct LiveChatClientBuilder<U, SF, ENF, CF, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    live_url: U,
    on_start: SF,
    on_end: ENF,
    on_chat: CF,
    on_error: ERF,
}

impl LiveChatClientBuilder<(), Empty, Empty, Empty, Empty> {
    pub fn new() -> Self {
        Self {
            live_url: (),
            on_start: Empty {},
            on_end: Empty {},
            on_chat: Empty {},
            on_error: Empty {},
        }
    }
}

impl Default for LiveChatClientBuilder<(), Empty, Empty, Empty, Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<U, ENF, CF, ERF> LiveChatClientBuilder<U, Empty, ENF, CF, ERF>
where
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    pub fn on_start<SF>(self, f: SF) -> LiveChatClientBuilder<U, SF, ENF, CF, ERF>
    where
        SF: Fn(String),
    {
        LiveChatClientBuilder {
            live_url: self.live_url,
            on_start: f,
            on_end: self.on_end,
            on_chat: self.on_chat,
            on_error: self.on_error,
        }
    }
}

impl<U, SF, CF, ERF> LiveChatClientBuilder<U, SF, Empty, CF, ERF>
where
    SF: InvokeOnStart,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    pub fn on_end<ENF>(self, f: ENF) -> LiveChatClientBuilder<U, SF, ENF, CF, ERF>
    where
        ENF: Fn(),
    {
        LiveChatClientBuilder {
            live_url: self.live_url,
            on_start: self.on_start,
            on_end: f,
            on_chat: self.on_chat,
            on_error: self.on_error,
        }
    }
}

impl<U, SF, ENF, ERF> LiveChatClientBuilder<U, SF, ENF, Empty, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    ERF: InvokeOnError,
{
    pub fn on_chat<CF>(self, f: CF) -> LiveChatClientBuilder<U, SF, ENF, CF, ERF>
    where
        CF: Fn(ChatItem),
    {
        LiveChatClientBuilder {
            live_url: self.live_url,
            on_start: self.on_start,
            on_end: self.on_end,
            on_chat: f,
            on_error: self.on_error,
        }
    }
}

impl<U, SF, ENF, CF> LiveChatClientBuilder<U, SF, ENF, CF, Empty>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
{
    pub fn on_error<ERF>(self, f: ERF) -> LiveChatClientBuilder<U, SF, ENF, CF, ERF>
    where
        ERF: Fn(anyhow::Error),
    {
        LiveChatClientBuilder {
            live_url: self.live_url,
            on_start: self.on_start,
            on_end: self.on_end,
            on_chat: self.on_chat,
            on_error: f,
        }
    }
}

impl<SF, ENF, CF, ERF> LiveChatClientBuilder<(), SF, ENF, CF, ERF>
where
    SF: InvokeOnStart,
    ENF: InvokeOnEnd,
    CF: InvokeOnChat,
    ERF: InvokeOnError,
{
    pub fn live_id(self, live_id: String) -> LiveChatClientBuilder<String, SF, ENF, CF, ERF> {
        LiveChatClientBuilder {
            live_url: format!("https://www.youtube.com/watch?v={}", live_id),
            on_start: self.on_start,
            on_end: self.on_end,
            on_chat: self.on_chat,
            on_error: self.on_error,
        }
    }

    pub fn url(
        self,
        raw_url: impl AsRef<str>,
    ) -> Result<LiveChatClientBuilder<String, SF, ENF, CF, ERF>, anyhow::Error> {
        Url::parse(raw_url.as_ref())?;
        Ok(LiveChatClientBuilder {
            live_url: raw_url.as_ref().to_string(),
            on_start: self.on_start,
            on_end: self.on_end,
            on_chat: self.on_chat,
            on_error: self.on_error,
        })
    }

    pub fn channel_id(self, channel_id: String) -> LiveChatClientBuilder<String, SF, ENF, CF, ERF> {
        LiveChatClientBuilder {
            live_url: format!("https://www.youtube.com/channel/{}/live", channel_id),
            on_start: self.on_start,
            on_end: self.on_end,
            on_chat: self.on_chat,
            on_error: self.on_error,
        }
    }
}

youtube_chat_macro::gen_builder!();

#[cfg(test)]
mod live_chat_tests {
    use super::*;

    #[test]
    fn test_builder() {
        let client = LiveChatClientBuilder::new()
            .live_id("_eM9C3zZL14".to_string())
            .build();
        assert_eq!(
            &client.live_url,
            "https://www.youtube.com/watch?v=_eM9C3zZL14"
        );

        let client = LiveChatClientBuilder::new()
            .channel_id("UCHVXbQzkl3rDfsXWo8xi2qw".to_string())
            .build();
        assert_eq!(
            &client.live_url,
            "https://www.youtube.com/channel/UCHVXbQzkl3rDfsXWo8xi2qw/live"
        );

        let client = LiveChatClientBuilder::new()
            .url("https://www.youtube.com/watch?v=Dx5qFachd3A")
            .unwrap()
            .on_chat(|_chat_item| println!("Hello"))
            .on_start(|live_id| println!("{}", live_id))
            .on_end(|| {})
            .build();
        assert_eq!(
            &client.live_url,
            "https://www.youtube.com/watch?v=Dx5qFachd3A"
        );

        let client = LiveChatClientBuilder::new()
            .url("https://www.youtube.com/watch?v=Dx5qFachd3A")
            .unwrap()
            .on_error(|_err| {})
            .on_chat(|_chat_item| println!("Hello"))
            .on_end(|| {})
            .build();
        assert_eq!(
            &client.live_url,
            "https://www.youtube.com/watch?v=Dx5qFachd3A"
        );
    }
}
