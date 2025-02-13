use super::User;

use serde::{Deserialize, Serialize};

/// Represents a [`ChatMember`](crate::types::ChatMember) that is under certain restrictions in the chat. Supergroups only.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberrestricted>
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatMemberRestricted {
    /// Information about the user
    pub user: User,
    /// `true`, if the user is a member of the chat at the moment of the request
    pub is_member: bool,
    /// `true`, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: bool,
    /// `true`, if the user is allowed to send audios
    pub can_send_audios: bool,
    /// `true`, if the user is allowed to send documents
    pub can_send_documents: bool,
    /// `true`, if the user is allowed to send photos
    pub can_send_photos: bool,
    /// `true`, if the user is allowed to send videos
    pub can_send_videos: bool,
    /// `true`, if the user is allowed to send video notes
    pub can_send_video_notes: bool,
    /// `true`, if the user is allowed to send voice notes
    pub can_send_voice_notes: bool,
    /// `true`, if the user is allowed to send polls
    pub can_send_polls: bool,
    /// `true`, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: bool,
    /// `true`, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: bool,
    /// `true`, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// `true`, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// `true`, if the user is allowed to pin messages
    pub can_pin_messages: bool,
    /// `true`, if the user is allowed to create forum topics
    pub can_manage_topics: bool,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is restricted forever
    pub until_date: i64,
}
