use chrono::{DateTime, Local};
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
    pub timestamp: DateTime<Local>,
}

#[derive(Debug)]
pub struct Author {
    pub name: String,
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
    pub alt: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmojiItem {
    #[serde(flatten)]
    pub image_item: ImageItem,
    pub emoji_text: String,
    pub is_custom_emoji: bool,
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
