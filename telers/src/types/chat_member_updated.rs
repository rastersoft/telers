use super::{Chat, ChatInviteLink, ChatMember, Update, UpdateKind, User};

use crate::{errors::ConvertToTypeError, FromEvent};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents changes in the status of a chat member.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberupdated>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromEvent)]
#[event(try_from = Update)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: i64,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
    /// `true`, if the user joined the chat after sending a direct join request without using an invite link and being approved by an administrator
    pub via_join_request: Option<bool>,
    /// `true`, if the user joined the chat via a chat folder invite link
    pub via_chat_folder_invite_link: Option<bool>,
}

impl TryFrom<Update> for ChatMemberUpdated {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::MyChatMember(val) | UpdateKind::ChatMember(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "ChatMemberUpdated")),
        }
    }
}
