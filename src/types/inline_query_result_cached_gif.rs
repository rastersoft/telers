use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with specified content instead of the animation.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedgif>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedGif {
    /// Type of the result, must be *gif*
    #[serde(rename = "type", default = "gif")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// *Optional*. Title for the result
    pub title: Option<String>,
    /// *Optional*. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedGif {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, gif_file_id: T) -> Self {
        Self {
            id: id.into(),
            gif_file_id: gif_file_id.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id(mut self, val: impl Into<String>) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn gif_file_id(mut self, val: impl Into<String>) -> Self {
        self.gif_file_id = val.into();
        self
    }

    #[must_use]
    pub fn title(mut self, val: impl Into<String>) -> Self {
        self.title = Some(val.into());
        self
    }

    #[must_use]
    pub fn caption(self, val: impl Into<String>) -> Self {
        Self {
            caption: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entity(self, val: MessageEntity) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}

impl Default for InlineQueryResultCachedGif {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: gif(),
            id: String::default(),
            gif_file_id: String::default(),
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

fn gif() -> String {
    InlineQueryResultType::Gif.into()
}
