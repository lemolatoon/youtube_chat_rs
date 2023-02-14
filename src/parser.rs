use crate::{
    item::{Author, Badge, ChatItem, EmojiItem, ImageItem, MessageItem, SuperChat},
    reqest::ReqestOptions,
    youtube_types::{
        Action, AuthorBadge, GetLiveChatResponse, LiveChatMembershipItemRenderer,
        LiveChatPaidMessageRenderer, LiveChatPaidStickerRenderer, LiveChatTextMessageRenderer,
        MessageRun, Thumbnail,
    },
};
use anyhow::anyhow;
use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;

fn get_options_from_live_page<'a>(
    data: &'a str,
) -> Result<(ReqestOptions<'a>, &'a str), anyhow::Error> {
    let live_id_regex =
        Regex::new(r#"<link rel="canonical" href="https:\/\/www.youtube.com\/watch\?v=(.+?)">"#)
            .unwrap();
    let live_id = match live_id_regex.find(&data) {
        Some(matched) => matched.as_str(),
        None => return Err(anyhow!("Live Stream was not found.")),
    };

    let replay_regex = Regex::new(r#"['"]isReplay['"]:\s*(true)"#).unwrap();
    match replay_regex.find(&data) {
        Some(_) => {}
        None => return Err(anyhow!("{live_id} is finished live.")),
    };

    let api_key_regex = Regex::new(r#"['"]INNERTUBE_API_KEY['"]:\s*['"](.+?)['"]"#).unwrap();
    let api_key = match api_key_regex.find(&data) {
        Some(matched) => matched.as_str(),
        None => return Err(anyhow!("{live_id} is finished live.")),
    };

    let client_version_regex = Regex::new(r#"['"]clientVersion['"]:\s*['"]([\d.]+?)['"]"#).unwrap();
    let client_version = match client_version_regex.find(&data) {
        Some(matched) => matched.as_str(),
        None => return Err(anyhow!("Client Version was not found.")),
    };

    let continuation_regex = Regex::new(r#"['"]continuation['"]:\s*['"](.+?)['"]"#).unwrap();
    let continuation = match continuation_regex.find(&data) {
        Some(matched) => matched.as_str(),
        None => return Err(anyhow!("Client Version was not found.")),
    };

    Ok((
        ReqestOptions {
            api_key,
            client_version,
            continuation,
        },
        live_id,
    ))
}

pub fn parse_chat_data(data: GetLiveChatResponse) -> (Vec<ChatItem>, String) {
    let chat_items = if !data
        .continuation_contents
        .live_chat_continuaton
        .actions
        .is_empty()
    {
        data.continuation_contents
            .live_chat_continuaton
            .actions
            .into_iter()
            .filter_map(parse_action_to_chat_item)
            .collect()
    } else {
        Vec::new()
    };
    let continuation_data = data
        .continuation_contents
        .live_chat_continuaton
        .continuations
        .into_iter()
        .next();
    let continuation = {
        if let Some(continuation_data) = continuation_data {
            if let Some(invalidation_continuation_data) =
                continuation_data.invalidation_continuation_data
            {
                invalidation_continuation_data.continuation
            } else if let Some(timed_continuation_data) = continuation_data.timed_continuation_data
            {
                timed_continuation_data.continuation
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    };
    (chat_items, continuation)
}

fn parse_action_to_chat_item(action: Action) -> Option<ChatItem> {
    let message_renderer = renderer_from_action(action)?;
    let message = message_renderer.runs();
    let author_name_text = message_renderer.author_name();
    let id = message_renderer.id();
    let thumbnail =
        parse_thumbnails_to_image_item(message_renderer.thumbnails(), author_name_text.clone());
    let channel_id = message_renderer.channel_id();
    let message = parse_message(message);
    let timestamp = message_renderer.time_stamp();
    let superchat = message_renderer.superchat();
    let mut chat_item = ChatItem {
        id,
        author: Author {
            name: author_name_text,
            thumbnail,
            channel_id,
            badge: None,
        },
        message,
        superchat,
        is_membership: false,
        is_verified: false,
        is_owner: false,
        is_moderator: false,
        timestamp,
    };
    message_renderer.process_badge(&mut chat_item);
    Some(chat_item)
}

pub enum Renderer {
    LiveChatTextMessageRenderer(LiveChatTextMessageRenderer),
    LiveChatPaidMessageRenderer(LiveChatPaidMessageRenderer),
    LiveChatMembershipItemRenderer(LiveChatMembershipItemRenderer),
    LiveChatPaidStickerRenderer(LiveChatPaidStickerRenderer),
}
impl Renderer {
    fn runs(&self) -> Vec<MessageRun> {
        match self {
            Renderer::LiveChatTextMessageRenderer(renderer) => renderer.message.runs.clone(),
            Renderer::LiveChatPaidMessageRenderer(renderer) => renderer
                .live_chat_text_message_renderer
                .message
                .runs
                .clone(),
            Renderer::LiveChatMembershipItemRenderer(renderer) => {
                renderer.header_sub_text.runs.clone()
            }
            Renderer::LiveChatPaidStickerRenderer(_) => Vec::new(),
        }
    }

    fn author_name(&self) -> Option<String> {
        match self {
            Renderer::LiveChatTextMessageRenderer(renderer) => renderer
                .message_renderer_base
                .author_name
                .clone()
                .map(|name| name.simple_text),
            Renderer::LiveChatPaidMessageRenderer(renderer) => renderer
                .live_chat_text_message_renderer
                .message_renderer_base
                .author_name
                .clone()
                .map(|name| name.simple_text),
            Renderer::LiveChatMembershipItemRenderer(renderer) => renderer
                .message_renderer_base
                .author_name
                .clone()
                .map(|name| name.simple_text),
            Renderer::LiveChatPaidStickerRenderer(renderer) => renderer
                .message_renderer_base
                .author_name
                .clone()
                .map(|name| name.simple_text),
        }
    }

    fn id(&self) -> String {
        match self {
            Renderer::LiveChatTextMessageRenderer(renderer) => {
                renderer.message_renderer_base.id.clone()
            }
            Renderer::LiveChatPaidMessageRenderer(renderer) => renderer
                .live_chat_text_message_renderer
                .message_renderer_base
                .id
                .clone(),
            Renderer::LiveChatMembershipItemRenderer(renderer) => {
                renderer.message_renderer_base.id.clone()
            }
            Renderer::LiveChatPaidStickerRenderer(renderer) => {
                renderer.message_renderer_base.id.clone()
            }
        }
    }

    fn thumbnails(&self) -> Vec<Thumbnail> {
        match self {
            Renderer::LiveChatTextMessageRenderer(renderer) => renderer
                .message_renderer_base
                .author_photo
                .thumbnails
                .clone(),
            Renderer::LiveChatPaidMessageRenderer(renderer) => renderer
                .live_chat_text_message_renderer
                .message_renderer_base
                .author_photo
                .thumbnails
                .clone(),
            Renderer::LiveChatMembershipItemRenderer(renderer) => renderer
                .message_renderer_base
                .author_photo
                .thumbnails
                .clone(),
            Renderer::LiveChatPaidStickerRenderer(renderer) => renderer
                .message_renderer_base
                .author_photo
                .thumbnails
                .clone(),
        }
    }

    fn channel_id(&self) -> String {
        match self {
            Renderer::LiveChatTextMessageRenderer(renderer) => renderer
                .message_renderer_base
                .author_external_channel_id
                .clone(),
            Renderer::LiveChatPaidMessageRenderer(renderer) => renderer
                .live_chat_text_message_renderer
                .message_renderer_base
                .author_external_channel_id
                .clone(),
            Renderer::LiveChatMembershipItemRenderer(renderer) => renderer
                .message_renderer_base
                .author_external_channel_id
                .clone(),
            Renderer::LiveChatPaidStickerRenderer(renderer) => renderer
                .message_renderer_base
                .author_external_channel_id
                .clone(),
        }
    }

    fn time_stamp(&self) -> Option<DateTime<Utc>> {
        let timestamp_usec = match self {
            Renderer::LiveChatTextMessageRenderer(renderer) => {
                renderer.message_renderer_base.timestamp_usec.clone()
            }
            Renderer::LiveChatPaidMessageRenderer(renderer) => renderer
                .live_chat_text_message_renderer
                .message_renderer_base
                .timestamp_usec
                .clone(),
            Renderer::LiveChatMembershipItemRenderer(renderer) => {
                renderer.message_renderer_base.timestamp_usec.clone()
            }
            Renderer::LiveChatPaidStickerRenderer(renderer) => {
                renderer.message_renderer_base.timestamp_usec.clone()
            }
        };
        Utc.timestamp_millis_opt(timestamp_usec.parse::<i64>().ok()?)
            .earliest()
    }

    fn author_badge(&self) -> Option<Vec<AuthorBadge>> {
        match self {
            Renderer::LiveChatTextMessageRenderer(renderer) => {
                renderer.message_renderer_base.author_badges.clone()
            }
            Renderer::LiveChatPaidMessageRenderer(renderer) => renderer
                .live_chat_text_message_renderer
                .message_renderer_base
                .author_badges
                .clone(),
            Renderer::LiveChatMembershipItemRenderer(renderer) => {
                renderer.message_renderer_base.author_badges.clone()
            }
            Renderer::LiveChatPaidStickerRenderer(renderer) => {
                renderer.message_renderer_base.author_badges.clone()
            }
        }
    }

    fn process_badge(&self, chat_item: &mut ChatItem) {
        if let Some(author_badges) = self.author_badge() {
            for author_badge in author_badges {
                let badge_renderer = author_badge.live_chat_author_badge_renderer;
                let icon_type = badge_renderer.icon.map(|icon| icon.icon_type);
                let tooltip = badge_renderer.tooltip.clone();
                if let Some(custom_thumbnail) = badge_renderer.custom_thumbnail {
                    let badge = (|| {
                        Some(Badge {
                            thumbnail: parse_thumbnails_to_image_item(
                                custom_thumbnail.thumbnails,
                                Some(tooltip.clone()),
                            )?,
                            label: tooltip.clone(),
                        })
                    })();
                    if let Some(badge) = badge {
                        chat_item.author.badge = Some(badge); // mutate
                    }
                    chat_item.is_membership = true; // mutate
                } else {
                    if icon_type == Some("OWNER".to_string()) {
                        chat_item.is_owner = true;
                    } else if icon_type == Some("VERIFIED".to_string()) {
                        chat_item.is_owner = true;
                    } else if icon_type == Some("MODERATOR".to_string()) {
                        chat_item.is_owner = true;
                    }
                }
            }
        }
    }

    fn superchat(&self) -> Option<SuperChat> {
        match self {
            Renderer::LiveChatTextMessageRenderer(_) => None,
            Renderer::LiveChatPaidMessageRenderer(renderer) => Some(SuperChat {
                amount: renderer.purchase_amount_text.simple_text.clone(),
                color: convert_color_to_hex6(renderer.body_background_color),
                sticker: None,
            }),
            Renderer::LiveChatMembershipItemRenderer(_) => None,
            Renderer::LiveChatPaidStickerRenderer(renderer) => Some(SuperChat {
                amount: renderer.purchase_amount_text.simple_text.clone(),
                color: convert_color_to_hex6(renderer.background_color),
                sticker: parse_thumbnails_to_image_item(
                    renderer.sticker.thumbnails.clone(),
                    Some(
                        renderer
                            .sticker
                            .accessibility
                            .accessibility_data
                            .label
                            .clone(),
                    ),
                ),
            }),
        }
    }
}
impl From<LiveChatTextMessageRenderer> for Renderer {
    fn from(value: LiveChatTextMessageRenderer) -> Self {
        Self::LiveChatTextMessageRenderer(value)
    }
}
impl From<LiveChatPaidMessageRenderer> for Renderer {
    fn from(value: LiveChatPaidMessageRenderer) -> Self {
        Self::LiveChatPaidMessageRenderer(value)
    }
}
impl From<LiveChatMembershipItemRenderer> for Renderer {
    fn from(value: LiveChatMembershipItemRenderer) -> Self {
        Self::LiveChatMembershipItemRenderer(value)
    }
}
impl From<LiveChatPaidStickerRenderer> for Renderer {
    fn from(value: LiveChatPaidStickerRenderer) -> Self {
        Self::LiveChatPaidStickerRenderer(value)
    }
}
fn renderer_from_action(action: Action) -> Option<Renderer> {
    let item = action.add_chat_item_action?.item;
    if let Some(renderer) = item.live_chat_text_message_renderer {
        Some(renderer.into())
    } else if let Some(renderer) = item.live_chat_paid_message_renderer {
        Some(renderer.into())
    } else if let Some(renderer) = item.live_chat_membership_item_renderer {
        Some(renderer.into())
    } else if let Some(renderer) = item.live_chat_paid_sticker_renderer {
        Some(renderer.into())
    } else {
        None
    }
}

fn parse_thumbnails_to_image_item(
    thumbnails: Vec<Thumbnail>,
    alt: Option<String>,
) -> Option<ImageItem> {
    let thumbnail = thumbnails.into_iter().next()?;
    Some(ImageItem {
        url: thumbnail.url,
        alt: alt?,
    })
}

fn convert_color_to_hex6(color_number: isize) -> String {
    format!(
        "#{}",
        color_number
            .to_ne_bytes()
            .into_iter()
            .take(4)
            .map(|x| format!("{:02X}", x))
            .collect::<String>()
    )
}

fn parse_message(runs: Vec<MessageRun>) -> Vec<MessageItem> {
    runs.into_iter()
        .map(|run| match run {
            MessageRun::MessageText { text } => MessageItem::Text(text),
            MessageRun::MessageEmoji {
                emoji,
                variant_ids: _,
                is_custome_emoji,
            } => {
                let thumbnail = emoji.image.thumbnails.into_iter().next();
                let shortcut = emoji.shortcuts.into_iter().next();
                let image_item = thumbnail
                    .zip(shortcut.clone())
                    .map(|(thumbnail, shortcut)| ImageItem {
                        url: thumbnail.url,
                        alt: shortcut,
                    });

                let emoji_text = if is_custome_emoji == Some(true) {
                    shortcut
                } else {
                    Some(emoji.emoji_id)
                };

                MessageItem::Emoji(EmojiItem {
                    image_item,
                    emoji_text,
                    is_custome_emoji,
                })
            }
        })
        .collect()
}
