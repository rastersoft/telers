use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes an inline message sent by a [`Web App`](https://core.telegram.org/bots/webapps) on behalf of a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#sentwebappmessage>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message. Available only if there is an [`inline keyboard`](https://core.telegram.org/bots/api#inlinekeyboardmarkup) attached to the message.
    pub inline_message_id: Option<Box<str>>,
}
