use std::marker::PhantomData;

use url::Url;

pub struct LiveChatClient<SF, ENF, CF, ERF>
where
    SF: Fn(),
    ENF: Fn(),
    CF: Fn(),
    ERF: Fn(),
{
    live_url: String,
    on_start: Option<SF>,
    on_end: Option<ENF>,
    on_chat: Option<CF>,
    on_error: Option<ERF>,
}

impl<SF, ENF, CF, ERF> LiveChatClient<SF, ENF, CF, ERF>
where
    SF: Fn(),
    ENF: Fn(),
    CF: Fn(),
    ERF: Fn(),
{
    pub fn start(&self) {
        unimplemented!()
    }
}

struct Empty;
struct Filled<T> {
    __phantom_data: PhantomData<T>,
}
pub struct LiveChatClientBuilder<U, SF, ENF, CF, ERF> {
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
            on_start: Empty,
            on_end: Empty,
            on_chat: Empty,
            on_error: Empty,
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

impl<SF, ENF, CF, ERF> LiveChatClientBuilder<String, SF, ENF, CF, ERF> {
    pub fn build(self) -> LiveChatClient<SF, ENF, CF, ERF> {
        LiveChatClient {
            live_url: self.live_url,
            on_start: self.on_start,
            on_end: self.on_end,
            on_chat: self.on_chat,
            on_error: self.on_error,
        }
    }
}

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
            .build();
        assert_eq!(
            &client.live_url,
            "https://www.youtube.com/watch?v=Dx5qFachd3A"
        );
    }
}
