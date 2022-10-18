use super::ChatIdKind;

use serde::{Deserialize, Serialize};

/// Represents the `scope <https://core.telegram.org/bots/api#botcommandscope>`_ of bot commands, covering a specific member of a group or supergroup chat.
/// <https://core.telegram.org/bots/api#botcommandscopechatmember>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatMember {
    /// Scope type, must be *chat_member*
    #[serde(rename = "type", default = "chat_member")]
    pub scope_type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format :code:`@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}

fn chat_member() -> String {
    "chat_member".to_string()
}
