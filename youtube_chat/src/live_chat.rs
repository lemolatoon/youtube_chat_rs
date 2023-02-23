use url::Url;

use crate::request::{fetch_live_page, RequestOptions};

/// SF, ENF, CF, ERF is `()` or `T: Fn()`
pub struct LiveChatClient<SF, ENF, CF, ERF> {
    live_url: String,
    on_start: SF,
    on_end: ENF,
    on_chat: CF,
    on_error: ERF,
    options: Option<RequestOptions>,
}

impl<SF, ENF, CF, ERF> LiveChatClient<SF, ENF, CF, ERF> {
    fn _start(&self) {}
    pub fn execute(&self) {}
}

impl<SF, ENF, CF, ERF> LiveChatClient<SF, ENF, CF, ERF>
where
    SF: Fn(String),
{
    pub async fn start(&mut self) {
        let result: Result<String, anyhow::Error> = async {
            let (options, live_id) = fetch_live_page(self.live_url).await?;
            Ok(live_id)
        }
        .await;
        match result {
            Ok(live_id) => self.invoke_on_start(live_id),
            Err(err) => self.invoke_on_error(err),
        }
    }
}
impl<ENF, CF, ERF> LiveChatClient<(), ENF, CF, ERF> {
    pub fn invoke_start(&self, live_id: String) {}
}

impl<ENF, SF, CF, ERF> LiveChatClient<SF, ENF, CF, ERF>
where
    SF: Fn(String),
{
    pub fn invoke_start(&self, live_id: String) {
        (self.on_start)(live_id)
    }
}

pub struct LiveChatClientBuilder<U, SF, ENF, CF, ERF> {
    live_url: U,
    on_start: SF,
    on_end: ENF,
    on_chat: CF,
    on_error: ERF,
}

impl LiveChatClientBuilder<(), (), (), (), ()> {
    pub fn new() -> Self {
        Self {
            live_url: (),
            on_start: (),
            on_end: (),
            on_chat: (),
            on_error: (),
        }
    }
}

impl<U, ENF, CF, ERF> LiveChatClientBuilder<U, (), ENF, CF, ERF> {
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

impl<U, SF, CF, ERF> LiveChatClientBuilder<U, SF, (), CF, ERF> {
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

impl<U, SF, ENF, ERF> LiveChatClientBuilder<U, SF, ENF, (), ERF> {
    pub fn on_chat<CF>(self, f: CF) -> LiveChatClientBuilder<U, SF, ENF, CF, ERF>
    where
        CF: Fn(),
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

impl<U, SF, ENF, CF> LiveChatClientBuilder<U, SF, ENF, CF, ()> {
    pub fn on_error<ERF>(self, f: ERF) -> LiveChatClientBuilder<U, SF, ENF, CF, ERF>
    where
        CF: Fn(),
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

impl<SF, ENF, CF, ERF> LiveChatClientBuilder<(), SF, ENF, CF, ERF> {
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
            .url("https://www.youtube.com/watch?v=Dx5qFachd3A".to_string())
            .unwrap()
            .on_chat(|| println!("Hello"))
            .build();
        assert_eq!(
            &client.live_url,
            "https://www.youtube.com/watch?v=Dx5qFachd3A"
        );
    }
}
