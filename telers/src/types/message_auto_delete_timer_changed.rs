use serde::{Deserialize, Serialize};

/// This object represents a service message about a change in auto-delete timer settings.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageautodeletetimerchanged>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat; in seconds
    pub message_auto_delete_time: i64,
}
