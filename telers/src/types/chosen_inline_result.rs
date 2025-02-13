use super::{Location, Update, UpdateKind, User};

use crate::{errors::ConvertToTypeError, FromEvent};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a [`result`](https://core.telegram.org/bots/api#inlinequeryresult) of an inline query that was chosen by the user and sent to their chat partner.
/// # Notes
/// It is necessary to enable [`inline feedback`](https://core.telegram.org/bots/inline#collecting-feedback) via [`@BotFather`](https://t.me/botfather) in order to receive these objects in updates.
/// # Documentation
/// <https://core.telegram.org/bots/api#choseninlineresult>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, FromEvent)]
#[event(try_from = Update)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: Box<str>,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message. Available only if there is an [`inline keyboard`](https://core.telegram.org/bots/api#inlinekeyboardmarkup) attached to the message. Will be also received in [`callback queries`](https://core.telegram.org/bots/api#callbackquery) and can be used to [`edit`](https://core.telegram.org/bots/api#updating-messages) the message.
    pub inline_message_id: Option<Box<str>>,
    /// The query that was used to obtain the result
    pub query: Box<str>,
}

impl TryFrom<Update> for ChosenInlineResult {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::ChosenInlineResult(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "ChosenInlineResult")),
        }
    }
}
