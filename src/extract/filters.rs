use crate::{
    client::Bot, context::Context, error::app::ExtractError, extract::FromEventAndContext,
    filters::CommandObject, types::Update,
};

use futures::future::{err, ok, Ready};
use std::{cell::RefCell, rc::Rc};

impl FromEventAndContext for CommandObject {
    type Error = ExtractError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, _: &Update, context: Rc<RefCell<Context>>) -> Self::Future {
        context.borrow().get("command").map_or(
            err(ExtractError {
                message: "Key `command` not found in the context".to_string(),
            }),
            |command| {
                command.downcast_ref::<CommandObject>().map_or(
                    err(ExtractError {
                        message: format!(
                            "Failed to downcast command, got `{:?}` instead `CommandObject`",
                            command
                        ),
                    }),
                    |command| ok(command.clone()),
                )
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::dispatcher::event::Handler;

    #[test]
    fn test_filters_extract() {
        fn assert_impl_handler<T: FromEventAndContext>(_: impl Handler<T>) {}

        assert_impl_handler(|_: CommandObject| async { unimplemented!() });
    }
}
