use std::marker::PhantomData;

use url::Url;

struct LiveChatClient {
    live_url: String,
}

impl LiveChatClient {
    pub fn start(&self) {}
}

struct Empty;
struct FullFilled;
pub struct LiveChatClientBuilder<T> {
    live_url: Option<String>,
    __live_id_state: PhantomData<T>,
}

impl LiveChatClientBuilder<Empty> {
    pub fn live_id(live_id: String) -> LiveChatClientBuilder<FullFilled> {
        LiveChatClientBuilder {
            live_url: Some(live_id),
            __live_id_state: PhantomData::<FullFilled> {},
        }
    }

    pub fn url(
        raw_url: impl AsRef<str>,
    ) -> Result<LiveChatClientBuilder<FullFilled>, anyhow::Error> {
        Url::parse(raw_url.as_ref())?;
        Ok(LiveChatClientBuilder {
            live_url: Some(raw_url.as_ref().to_string()),
            __live_id_state: PhantomData::<FullFilled> {},
        })
    }

    pub fn channel_id(channel_id: String) -> LiveChatClientBuilder<FullFilled> {
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
