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
struct GetLiveChatBodyContext {
    client: GetLiveChatBodyContextClient,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetLiveChatBodyContextClient {
    #[serde(rename = "clientVersion")]
    client_version: String,
    #[serde(rename = "clientName")]
    client_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetLiveChatResponse {
    #[serde(rename = "responseContext")]
    response_context: serde_json::Value,
    #[serde(rename = "trackingParams")]
    tracking_params: Option<String>,
    #[serde(rename = "continuationContents")]
    continuation_contents: GetLiveChatResponseContinuationContents,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetLiveChatResponseContinuationContents {
    continuations: Vec<Continuation>,
    actions: Vec<Action>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Continuation {
    #[serde(rename = "invalidationContinuationData")]
    invalidation_continuation_data: Option<InvalidationContinuationData>,
    #[serde(rename = "timedContinuationData")]
    timed_continuation_data: Option<TimedContinuationData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InvalidationContinuationData {
    #[serde(rename = "invalidationId")]
    invalidation_id: InvalidationId,
    #[serde(rename = "timeoutMs")]
    timeout_ms: usize,
    continuation: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InvalidationId {
    #[serde(rename = "objectSource")]
    object_source: usize,
    #[serde(rename = "objectId")]
    object_id: String,
    topic: String,
    #[serde(rename = "subscribeToGcmTopics")]
    subscribe_to_gcm_topics: bool,
    #[serde(rename = "protoCreationTimestampMs")]
    proto_creation_timestamp_ms: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct TimedContinuationData {
    #[serde(rename = "timeoutMs")]
    timeout_ms: usize,
    continuation: String,
    #[serde(rename = "clickTrackingParams")]
    click_tracking_params: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Action {
    #[serde(rename = "addChatItemAction")]
    add_chat_item_action: Option<AddChatItemAction>,
    #[serde(rename = "addLiveChatTickerItemAction")]
    add_live_chat_ticker_item_action: Option<serde_json::Value>,
}

/* MessageRun */
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum MessageRun {
    MessageText {
        text: String,
    },
    MessageEmoji {
        emoji: Emoji,
        #[serde(rename = "variantIds")]
        variant_ids: Vec<String>,
        #[serde(rename = "isCustomeEmoji")]
        is_custome_emoji: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Emoji {
    #[serde(rename = "emojiId")]
    emoji_id: String,
    shortcuts: Vec<String>,
    #[serde(rename = "searchTerms")]
    search_terms: Vec<String>,
    #[serde(rename = "supportsSkinTone")]
    supports_skin_tone: bool,
    image: Image,
}
#[derive(Serialize, Deserialize, Debug)]
struct Image {
    thumbnails: Vec<Thumbnail>,
    accessibility: Accessibility,
}
#[derive(Serialize, Deserialize, Debug)]
struct Accessibility {
    #[serde(rename = "accessibilityData")]
    accessibility_data: AccessibilityData,
}
#[derive(Serialize, Deserialize, Debug)]
struct AccessibilityData {
    label: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Thumbnail {
    url: String,
    width: Option<usize>,
    height: Option<usize>,
}
/* MessageRun End */

/* MessageRenderers */
/* MessageRenderersBase */
/* AuthorBadge */
#[derive(Serialize, Deserialize, Debug)]
struct AuthorBadge {
    #[serde(rename = "liveChatAuthorBadgeRenderer")]
    live_chat_author_badge_renderer: LiveChatAuthorBadgeRenderer,
}
#[derive(Serialize, Deserialize, Debug)]
struct LiveChatAuthorBadgeRenderer {
    #[serde(rename = "customThumbnail")]
    custom_thumbnail: Option<CustomThumbnail>,
    icon: Option<Icon>,
    tooltip: String,
    accessibility: Accessibility,
}
#[derive(Serialize, Deserialize, Debug)]
struct CustomThumbnail {
    thumbnails: Vec<Thumbnail>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Icon {
    #[serde(rename = "iconType")]
    icon_type: String,
}
/* AuthorBadge End */

#[derive(Serialize, Deserialize, Debug)]
struct MessageRendererBase {
    #[serde(rename = "authorName")]
    author_name: Option<AuthorName>,
    #[serde(rename = "authorPhoto")]
    author_photo: AuthorPhoto,
    #[serde(rename = "authorBadges")]
    author_badges: Option<Vec<AuthorBadge>>,
    #[serde(rename = "contextMenuEndpoint")]
    context_menu_endpoint: ContextMenuEndpoint,
    id: String,
    #[serde(rename = "timestampUsec")]
    timestamp_usec: String,
    #[serde(rename = "authorExternalChannelId")]
    author_external_channel_id: String,
    #[serde(rename = "contextMenuAccessibility")]
    context_menu_accessibility: Accessibility,
}
#[derive(Serialize, Deserialize, Debug)]
struct ContextMenuEndpoint {
    #[serde(rename = "clickTrackingParams")]
    click_tracking_params: String,
    #[serde(rename = "commandMetadata")]
    command_metadata: CommandMetadata,
    #[serde(rename = "liveChatItemContextMenuEndpoint")]
    live_chat_item_context_menu_endpoint: LiveChatItemContextMenuEndpoint,
}
#[derive(Serialize, Deserialize, Debug)]
struct LiveChatItemContextMenuEndpoint {
    params: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct CommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    web_command_metadata: WebCommandMetadata,
}
#[derive(Serialize, Deserialize, Debug)]
struct WebCommandMetadata {
    #[serde(rename = "ignoreNavigation")]
    ignore_navigation: bool,
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthorPhoto {
    thumbnails: Vec<Thumbnail>,
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthorName {
    #[serde(rename = "simpleText")]
    simple_text: String,
}
/* MessageRenderersBase End */
#[derive(Serialize, Deserialize, Debug)]
struct LiveChatTextMessageRenderer {
    #[serde(flatten)]
    message_renderer_base: MessageRendererBase,
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    runs: Vec<MessageRun>,
}
#[derive(Serialize, Deserialize, Debug)]
struct LiveChatPaidMessageRenderer {
    #[serde(flatten)]
    live_chat_text_message_renderer: LiveChatTextMessageRenderer,
    #[serde(rename = "purchaseAmountText")]
    purchase_amount_text: PurchaseAmountText,
    #[serde(rename = "headerBackgroundColor")]
    header_background_color: isize,
    #[serde(rename = "headerTextColor")]
    header_text_color: isize,
    #[serde(rename = "bodyBackgroundColor")]
    body_background_color: isize,
    #[serde(rename = "bodyTextColor")]
    body_text_color: isize,
    #[serde(rename = "authorNameTextColor")]
    author_name_text_color: isize,
}
#[derive(Serialize, Deserialize, Debug)]
struct LiveChatPaidStickerRenderer {
    #[serde(flatten)]
    message_renderer_base: MessageRendererBase,
    #[serde(rename = "purchaseAmountText")]
    purchase_amount_text: PurchaseAmountText,
    sticker: Sticker,
    #[serde(rename = "moneyChipBackgroundColor")]
    money_chip_background_color: isize,
    #[serde(rename = "moneyChipTextColor")]
    money_chip_text_color: isize,
    #[serde(rename = "stickerDisplayWidth")]
    sticker_display_width: isize,
    #[serde(rename = "stickerDisplayHeight")]
    sticker_display_height: isize,
    #[serde(rename = "BackgroundColor")]
    background_color: isize,
    #[serde(rename = "authorNameTextColor")]
    author_name_text_color: isize,
}
#[derive(Serialize, Deserialize, Debug)]
struct Sticker {
    thumbnails: Vec<Thumbnail>,
    accessibility: Accessibility,
}
#[derive(Serialize, Deserialize, Debug)]
struct PurchaseAmountText {
    #[serde(rename = "simpleText")]
    simple_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LiveChatMembershipItemRenderer {
    #[serde(rename = "headerSubText")]
    header_sub_text: HeaderSubText,
    #[serde(rename = "authorBadges")]
    author_badges: Vec<AuthorBadge>,
}
#[derive(Serialize, Deserialize, Debug)]
struct HeaderSubText {
    runs: Vec<MessageRun>,
}
#[derive(Serialize, Deserialize, Debug)]
struct AddChatItemAction {
    item: ActionItem,
    #[serde(rename = "clientId")]
    client_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ActionItem {
    #[serde(rename = "liveChatTextMessageRenderer")]
    live_chat_text_message_renderer: Option<LiveChatTextMessageRenderer>,
    #[serde(rename = "liveChatPaidMessageRenderer")]
    live_chat_paid_message_renderer: Option<LiveChatPaidMessageRenderer>,
    #[serde(rename = "liveChatMembershipItemRenderer")]
    live_chat_membership_item_renderer: Option<LiveChatMembershipItemRenderer>,
    #[serde(rename = "liveChatPaidStickerRenderer")]
    live_chat_paid_sticker_renderer: Option<LiveChatPaidStickerRenderer>,
    #[serde(rename = "liveChatViewerEngagementMessageRenderer")]
    live_chat_viewer_engagement_message_renderer: serde_json::Value,
}
