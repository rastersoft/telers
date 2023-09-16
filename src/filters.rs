//! Filters are main part of the library used to filter incoming updates and allow call handlers by their data (text, chat, user, command, etc.),
//! [`context`] (state, db, etc.) and other conditions.
//!
//! [`Filter`] is a trait that accepts [`bot`], [`update`] and [`context`] and returns `true` if the filter passes, otherwise `false`.
//! You can use [`Filter`] trait to create your own filters or use one of the ready-made implementations.
//! Most likely you will have to write your filters, so we recommend you to check out the [`examples/uppercase_filter`] to see how to create your own filters
//! and check ready-made implementations.
//!
//! Filters can be combined with logical operators [`And`] and [`Or`] and inverted with [`Invert`].
//! Each filter has a method [`Filter::invert`], [`Filter::and`] and [`Filter::or`] to create [`Invert`], [`And`] and [`Or`] filters respectively.
//!
//! Ready-made implementations:
//! * [`ChatType`]:
//! Filter for checking the type of chat.
//! Usually used with [`ChatTypeEnum`] (or its string representation) to check the type of chat.
//! Creates with `one` or `many` methods.
//! * [`Command`]:
//! This filter checks if the message is a command.
//! Filter accepts [`PatternType`] that represents a command pattern type for verification,
//! for example, text, [`BotCommand`] (just alias to text of command) or [`Regex`].
//! You can create a filter with `new` method with transferring all necessary data at once, or use [`CommandBuilder`] to create a filter step by step.
//! Instead of [`CommandBuilder`] you can use [`Command`] `one`, `one_with_prefix`, `many`, `many_with_prefix` methods.
//! * [`ContentType`]:
//! Filter for checking the type of content.
//! Usually used with [`ContentTypeEnum`] (or its string representation) to check the type of content.
//! Creates with `one` or `many` methods.
//! * [`State`]:
//! Filter for checking the state of the user/chat/etc.
//! Filter accepts [`StateType`] that represents a state type for verification,
//! for example, equal, any or none.
//! You can create a filter with `one` or `many` if you want to check the state with the exact value
//! or use `any` or `none` if you want to check the state with any value or without state, respectively.
//!
//! [`context`]: crate::context::Context
//! [`ChatTypeEnum`]: crate::enums::ChatType
//! [`ContentTypeEnum`]: crate::enums::ContentType

pub mod base;
pub mod chat_type;
pub mod command;
pub mod content_type;
pub mod logical;
pub mod state;
pub mod text;
pub mod user;

pub use base::Filter;
pub use chat_type::ChatType;
pub use command::{Command, CommandBuilder, CommandObject};
pub use content_type::ContentType;
pub use logical::{And, Invert, Or};
pub use state::{State, StateType};
pub use text::Text;
pub use user::User;
