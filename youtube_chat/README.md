# how to use

## build client

### build from `url` or `live_id` or `channel_id`
```rust
// pattern 1
let mut client = LiveClientBuilder::new()
    .url("https://www.youtube.com/watch?v=jfKfPfyJRdk".to_string())
    .unwrap()
    .build();
// pattern 2
let mut client = LiveChatClientBuilder::new()
    .live_id("jfKfPfyJRd".to_string())
    .build();
// pattern 3
let mut client = LiveChatClientBuilder::new()
    .channel_id("UCHVXbQzkl3rDfsXWo8xi2qw".to_string())
    .build();
```

### add callback function (each callback function is optional)
 - on_start
 - on_chat
 - on_end
 - on_error
```rust
let mut client = LiveChatClientBuilder::new()
    .url("https://www.youtube.com/watch?v=Dx5qFachd3A".to_string())
    .unwrap()
    .on_start(|_live_id| {})
    .on_error(|_err| {})
    .on_chat(|_chat_item| {})
    .on_end(|| {})
    .build();
```

### get ready for fetching live
```rust
client.start().await.unwrap();
```

### fetch chat comments
```rust
client.execute().await;
```

### call `execute` intervally if you want to fetch comments in real time
Example using tokio
```rust
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
```