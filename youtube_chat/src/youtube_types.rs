use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLiveChatBody {
    context: GetLiveChatBodyContext,
    continuation: String,
}

impl GetLiveChatBody {
    pub const fn new(continuation: String, client_version: String, client_name: String) -> Self {
        Self {
            context: GetLiveChatBodyContext {
                client: GetLiveChatBodyContextClient {
                    client_version,
                    client_name,
                },
            },
            continuation,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLiveChatBodyContext {
    client: GetLiveChatBodyContextClient,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLiveChatBodyContextClient {
    #[serde(rename = "clientVersion")]
    client_version: String,
    #[serde(rename = "clientName")]
    client_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLiveChatResponse {
    #[serde(rename = "responseContext")]
    pub response_context: serde_json::Value,
    #[serde(rename = "trackingParams")]
    pub tracking_params: Option<String>,
    #[serde(rename = "continuationContents")]
    pub continuation_contents: GetLiveChatResponseContinuationContents,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLiveChatResponseContinuationContents {
    #[serde(rename = "liveChatContinuation")]
    pub live_chat_continuaton: LiveChatContinuation,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LiveChatContinuation {
    pub continuations: Vec<Continuation>,
    pub actions: Option<Vec<Action>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Continuation {
    #[serde(rename = "invalidationContinuationData")]
    pub invalidation_continuation_data: Option<InvalidationContinuationData>,
    #[serde(rename = "timedContinuationData")]
    pub timed_continuation_data: Option<TimedContinuationData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvalidationContinuationData {
    #[serde(rename = "invalidationId")]
    pub invalidation_id: InvalidationId,
    #[serde(rename = "timeoutMs")]
    pub timeout_ms: usize,
    pub continuation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvalidationId {
    #[serde(rename = "objectSource")]
    pub object_source: usize,
    #[serde(rename = "objectId")]
    pub object_id: String,
    pub topic: String,
    #[serde(rename = "subscribeToGcmTopics")]
    pub subscribe_to_gcm_topics: bool,
    #[serde(rename = "protoCreationTimestampMs")]
    pub proto_creation_timestamp_ms: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TimedContinuationData {
    #[serde(rename = "timeoutMs")]
    pub timeout_ms: usize,
    pub continuation: String,
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    #[serde(rename = "addChatItemAction")]
    pub add_chat_item_action: Option<AddChatItemAction>,
    #[serde(rename = "addLiveChatTickerItemAction")]
    pub add_live_chat_ticker_item_action: Option<serde_json::Value>,
}

/* MessageRun */
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageRun {
    MessageText {
        text: String,
    },
    MessageEmoji {
        emoji: Emoji,
        #[serde(rename = "variantIds")]
        variant_ids: Option<Vec<String>>,
        #[serde(rename = "isCustomeEmoji")]
        is_custome_emoji: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
    pub shortcuts: Option<Vec<String>>,
    #[serde(rename = "searchTerms")]
    pub search_terms: Option<Vec<String>>,
    #[serde(rename = "supportsSkinTone")]
    pub supports_skin_tone: Option<bool>,
    pub image: Image,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub thumbnails: Vec<Thumbnail>,
    pub accessibility: Accessibility,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Accessibility {
    #[serde(rename = "accessibilityData")]
    pub accessibility_data: AccessibilityData,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessibilityData {
    pub label: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<usize>,
    pub height: Option<usize>,
}
/* MessageRun End */

/* MessageRenderers */
/* MessageRenderersBase */
/* AuthorBadge */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorBadge {
    #[serde(rename = "liveChatAuthorBadgeRenderer")]
    pub live_chat_author_badge_renderer: LiveChatAuthorBadgeRenderer,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiveChatAuthorBadgeRenderer {
    #[serde(rename = "customThumbnail")]
    pub custom_thumbnail: Option<CustomThumbnail>,
    pub icon: Option<Icon>,
    pub tooltip: String,
    pub accessibility: Accessibility,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomThumbnail {
    pub thumbnails: Vec<Thumbnail>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Icon {
    #[serde(rename = "iconType")]
    pub icon_type: String,
}
/* AuthorBadge End */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageRendererBase {
    #[serde(rename = "authorName")]
    pub author_name: Option<AuthorName>,
    #[serde(rename = "authorPhoto")]
    pub author_photo: AuthorPhoto,
    #[serde(rename = "authorBadges")]
    pub author_badges: Option<Vec<AuthorBadge>>,
    #[serde(rename = "contextMenuEndpoint")]
    pub context_menu_endpoint: ContextMenuEndpoint,
    pub id: String,
    #[serde(rename = "timestampUsec")]
    pub timestamp_usec: String,
    #[serde(rename = "authorExternalChannelId")]
    pub author_external_channel_id: String,
    #[serde(rename = "contextMenuAccessibility")]
    pub context_menu_accessibility: Accessibility,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContextMenuEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: Option<String>,
    #[serde(rename = "commandMetadata")]
    pub command_metadata: CommandMetadata,
    #[serde(rename = "liveChatItemContextMenuEndpoint")]
    pub live_chat_item_context_menu_endpoint: LiveChatItemContextMenuEndpoint,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiveChatItemContextMenuEndpoint {
    pub params: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: WebCommandMetadata,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebCommandMetadata {
    #[serde(rename = "ignoreNavigation")]
    pub ignore_navigation: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorPhoto {
    pub thumbnails: Vec<Thumbnail>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorName {
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}
/* MessageRenderersBase End */
#[derive(Serialize, Deserialize, Debug)]
pub struct LiveChatTextMessageRenderer {
    #[serde(flatten)]
    pub message_renderer_base: MessageRendererBase,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub runs: Vec<MessageRun>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LiveChatPaidMessageRenderer {
    #[serde(flatten)]
    pub live_chat_text_message_renderer: LiveChatTextMessageRenderer,
    #[serde(rename = "purchaseAmountText")]
    pub purchase_amount_text: PurchaseAmountText,
    #[serde(rename = "headerBackgroundColor")]
    pub header_background_color: isize,
    #[serde(rename = "headerTextColor")]
    pub header_text_color: isize,
    #[serde(rename = "bodyBackgroundColor")]
    pub body_background_color: isize,
    #[serde(rename = "bodyTextColor")]
    pub body_text_color: isize,
    #[serde(rename = "authorNameTextColor")]
    pub author_name_text_color: isize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LiveChatPaidStickerRenderer {
    #[serde(flatten)]
    pub message_renderer_base: MessageRendererBase,
    #[serde(rename = "purchaseAmountText")]
    pub purchase_amount_text: PurchaseAmountText,
    pub sticker: Sticker,
    #[serde(rename = "moneyChipBackgroundColor")]
    pub money_chip_background_color: isize,
    #[serde(rename = "moneyChipTextColor")]
    pub money_chip_text_color: isize,
    #[serde(rename = "stickerDisplayWidth")]
    pub sticker_display_width: isize,
    #[serde(rename = "stickerDisplayHeight")]
    pub sticker_display_height: isize,
    #[serde(rename = "BackgroundColor")]
    pub background_color: isize,
    #[serde(rename = "authorNameTextColor")]
    pub author_name_text_color: isize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub thumbnails: Vec<Thumbnail>,
    pub accessibility: Accessibility,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseAmountText {
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveChatMembershipItemRenderer {
    #[serde(flatten)]
    pub message_renderer_base: MessageRendererBase,
    #[serde(rename = "headerSubText")]
    pub header_sub_text: Option<HeaderSubText>,
    #[serde(rename = "authorBadges")]
    pub author_badges: Vec<AuthorBadge>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HeaderSubText {
    pub runs: Vec<MessageRun>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AddChatItemAction {
    pub item: ActionItem,
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionItem {
    #[serde(rename = "liveChatTextMessageRenderer")]
    pub live_chat_text_message_renderer: Option<LiveChatTextMessageRenderer>,
    #[serde(rename = "liveChatPaidMessageRenderer")]
    pub live_chat_paid_message_renderer: Option<LiveChatPaidMessageRenderer>,
    #[serde(rename = "liveChatMembershipItemRenderer")]
    pub live_chat_membership_item_renderer: Option<LiveChatMembershipItemRenderer>,
    #[serde(rename = "liveChatPaidStickerRenderer")]
    pub live_chat_paid_sticker_renderer: Option<LiveChatPaidStickerRenderer>,
    #[serde(rename = "liveChatViewerEngagementMessageRenderer")]
    pub live_chat_viewer_engagement_message_renderer: Option<serde_json::Value>,
}
