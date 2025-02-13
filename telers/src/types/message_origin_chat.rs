use super::Chat;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The message was originally sent on behalf of a chat to a group chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginchat>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MessageOriginChat {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Chat that sent the message originally
    pub sender_chat: Chat,
    /// For messages originally sent by an anonymous chat administrator, original message author signature
    pub author_signature: Option<Box<str>>,
}
