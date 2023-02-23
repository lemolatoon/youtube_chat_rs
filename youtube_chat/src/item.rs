use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};

#[derive(Debug)]
pub struct ChatItem {
    pub id: String,
    pub author: Author,
    pub message: Vec<MessageItem>,
    pub superchat: Option<SuperChat>,
    pub is_membership: bool,
    pub is_verified: bool,
    pub is_owner: bool,
    pub is_moderator: bool,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Author {
    pub name: Option<String>,
    pub thumbnail: Option<ImageItem>,
    pub channel_id: String,
    pub badge: Option<Badge>,
}

#[derive(Debug)]
pub enum MessageItem {
    Text(String),
    Emoji(EmojiItem),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageItem {
    pub url: String,
    pub alt: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmojiItem {
    #[serde(flatten)]
    pub image_item: Option<ImageItem>,
    pub emoji_text: Option<String>,
    pub is_custome_emoji: Option<bool>,
}
#[derive(Debug)]
pub struct Badge {
    pub thumbnail: ImageItem,
    pub label: String,
}

#[derive(Debug)]
pub struct SuperChat {
    pub amount: String,
    pub color: String,
    pub sticker: Option<ImageItem>,
}
