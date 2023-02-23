use std::time::Duration;

use tokio::{task, time};
use youtube_chat::live_chat::LiveChatClientBuilder;

#[tokio::main]
async fn main() {
    let mut client = LiveChatClientBuilder::new()
        .url("https://www.youtube.com/watch?v=jfKfPfyJRdk".to_string())
        .unwrap()
        .on_chat(|chat_item| println!("{:?}", chat_item.message))
        .on_error(|error| eprintln!("{:?}", error))
        .build();
    client.start().await.unwrap();
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(3000));
        loop {
            interval.tick().await;
            client.execute().await;
        }
    });

    forever.await.unwrap();
}
