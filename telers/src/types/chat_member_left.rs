use super::User;

use serde::{Deserialize, Serialize};

/// Represents a [`ChatMember`](crate::types::ChatMember) that isn't currently a member of the chat, but may join it themselves.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberleft>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatMemberLeft {
    /// Information about the user
    pub user: User,
}
