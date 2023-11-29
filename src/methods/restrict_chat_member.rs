use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatPermissions},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass `true` for all permissions to lift restrictions from a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#restrictchatmember>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct RestrictChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    /// Pass `true` if chat permissions are set independently. Otherwise, the `can_send_other_messages` and `can_add_web_page_previews` permissions will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions; the `can_send_polls` permission will imply the `can_send_messages` permission.
    pub use_independent_chat_permissions: Option<bool>,
    /// Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    pub until_date: Option<i64>,
}

impl RestrictChatMember {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, user_id: i64, permissions: ChatPermissions) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            permissions,
            use_independent_chat_permissions: None,
            until_date: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn permissions(self, val: ChatPermissions) -> Self {
        Self {
            permissions: val,
            ..self
        }
    }

    #[must_use]
    pub fn use_independent_chat_permissions(self, val: bool) -> Self {
        Self {
            use_independent_chat_permissions: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn until_date(self, val: i64) -> Self {
        Self {
            until_date: Some(val),
            ..self
        }
    }
}

impl RestrictChatMember {
    #[must_use]
    pub fn use_independent_chat_permissions_option(self, val: Option<bool>) -> Self {
        Self {
            use_independent_chat_permissions: val,
            ..self
        }
    }

    #[must_use]
    pub fn until_date_option(self, val: Option<i64>) -> Self {
        Self {
            until_date: val,
            ..self
        }
    }
}

impl TelegramMethod for RestrictChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("restrictChatMember", self, None)
    }
}

impl AsRef<RestrictChatMember> for RestrictChatMember {
    fn as_ref(&self) -> &Self {
        self
    }
}
