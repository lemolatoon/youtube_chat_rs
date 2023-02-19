use std::marker::PhantomData;

use url::Url;

struct LiveChatClient {
    live_url: String,
}

impl LiveChatClient {
    pub fn start(&self) {
        unimplemented!()
    }
}

struct Empty;
struct FullFilled;
pub struct LiveChatClientBuilder<T> {
    live_url: Option<String>,
    __live_id_state: PhantomData<T>,
}

impl LiveChatClientBuilder<Empty> {
    pub fn new() -> LiveChatClientBuilder<Empty> {
        Self {
            live_url: None,
            __live_id_state: PhantomData::<Empty> {},
        }
    }

    pub fn live_id(&self, live_id: String) -> LiveChatClientBuilder<FullFilled> {
        LiveChatClientBuilder {
            live_url: Some(format!("https://www.youtube.com/watch?v={}", live_id)),
            __live_id_state: PhantomData::<FullFilled> {},
        }
    }

    pub fn url(
        &self,
        raw_url: impl AsRef<str>,
    ) -> Result<LiveChatClientBuilder<FullFilled>, anyhow::Error> {
        Url::parse(raw_url.as_ref())?;
        Ok(LiveChatClientBuilder {
            live_url: Some(raw_url.as_ref().to_string()),
            __live_id_state: PhantomData::<FullFilled> {},
        })
    }

    pub fn channel_id(&self, channel_id: String) -> LiveChatClientBuilder<FullFilled> {
        LiveChatClientBuilder::<FullFilled> {
            live_url: Some(format!(
                "https://www.youtube.com/channel/{}/live",
                channel_id
            )),
            __live_id_state: PhantomData::<FullFilled> {},
        }
    }
}

impl LiveChatClientBuilder<FullFilled> {
    pub fn build(self) -> LiveChatClient {
        LiveChatClient {
            live_url: self.live_url.unwrap(), // never fails
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
