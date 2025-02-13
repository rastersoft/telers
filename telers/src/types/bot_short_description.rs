use serde::{Deserialize, Serialize};

/// This object represents the bot's short description.
/// # Documentation
/// <https://core.telegram.org/bots/api#botshortdescription>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotShortDescription {
    /// The bot's short description
    pub short_description: Box<str>,
}
