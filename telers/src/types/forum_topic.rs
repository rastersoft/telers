use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a forum topic.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopic>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: i64,
    /// Name of the topic
    pub name: Box<str>,
    /// Color of the topic icon in RGB format
    pub icon_color: Box<str>,
    /// Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: Option<Box<str>>,
}
