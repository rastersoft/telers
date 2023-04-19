use crate::{client::Bot, context::Context, error::ExtractionError, types::Update};

use std::{convert::Infallible, pin::Pin, sync::Arc};

/// Trait for extracting data from [`Update`] and [`Context`] to handlers arguments
pub trait FromEventAndContext<Client>: Sized {
    type Error: Into<ExtractionError>;

    /// Extracts data from [`Update`], [`Context`] and [`Bot`] to handler argument
    /// # Returns
    /// [`Ok(Self)`] if extraction was successful,
    /// [`Err(Self::Error)`] otherwise
    /// # Errors
    /// [`ExtractionError`] if extraction was unsuccessful
    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error>;
}

/// To be able to use [`Option`] as handler argument
/// This implementation will return [`None`] if extraction was unsuccessful, and [`Some(value)`] otherwise
/// Useful for optional arguments
impl<Client, T: FromEventAndContext<Client>> FromEventAndContext<Client> for Option<T> {
    type Error = Infallible;

    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        match T::extract(bot, update, context) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}

/// To be able to use [`Result`] as handler argument
/// This implementation will return [`Ok(value)`] if extraction was successful, and [`Err(error)`] otherwise,
/// where `error` is `T::Error` converted to `E`
/// Useful for optional arguments
impl<Client, T, E> FromEventAndContext<Client> for Result<T, E>
where
    T: FromEventAndContext<Client>,
    T::Error: Into<E>,
{
    type Error = Infallible;

    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(T::extract(bot, update, context).map_err(Into::into))
    }
}

/// To be able to use [`Box`] as handler argument
/// This implementation will return [`Box(value)`] if extraction was successful, and [`Err(error)`] otherwise
/// Useful for arguments with dynamic size
impl<Client, T: ?Sized> FromEventAndContext<Client> for Box<T>
where
    T: FromEventAndContext<Client>,
{
    type Error = T::Error;

    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        T::extract(bot, update, context).map(Box::new)
    }
}

/// To be able to use [`Pin<Box>`] as handler argument
/// This implementation will return [`Pin(value)`] if extraction was successful, and [`Err(error)`] otherwise
/// Useful for arguments with dynamic size, which should be pinned
impl<Client, T: ?Sized> FromEventAndContext<Client> for Pin<Box<T>>
where
    T: FromEventAndContext<Client>,
{
    type Error = T::Error;

    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        T::extract(bot, update, context).map(Box::pin)
    }
}

/// To be able to use handler without arguments
/// Handler without arguments will be called with [`()`] argument (unit type)
impl<Client> FromEventAndContext<Client> for () {
    type Error = Infallible;

    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(())
    }
}

#[allow(non_snake_case)]
mod factory_from_event_and_context {
    use super::{Arc, Bot, Context, ExtractionError, FromEventAndContext, Update};

    /// [`FromEventAndContext`] implementation for tuple arguments, which implements [`FromEventAndContext`]
    /// for each of its arguments, and returns [`Ok((value1, value2, ...))`] if extraction was successful,
    /// and [`Err(error)`] otherwise, where `error` is `T::Error` converted to [`ExtractionError`]
    macro_rules! factory ({ $($param:ident)* } => {
        impl<Client, $($param: FromEventAndContext<Client>,)*> FromEventAndContext<Client> for ($($param,)*) {
            type Error = ExtractionError;

            #[inline]
            fn extract(bot: Arc<Bot<Client>>, update: Arc<Update>, context: Arc<Context>) -> Result<Self, Self::Error> {
                // If any of the arguments fails to extract, the whole extraction fails
                Ok(($($param::extract(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context)).map_err(Into::into)?,)*))
            }
        }
    });

    // To be able to extract tuple with 1 arguments
    factory! { A }
    // To be able to extract tuple with 2 arguments
    factory! { A B }
    // To be able to extract tuple with 3 arguments
    factory! { A B C }
    // To be able to extract tuple with 4 arguments
    factory! { A B C D }
    // To be able to extract tuple with 5 arguments
    factory! { A B C D E}
    // To be able to extract tuple with 6 arguments
    factory! { A B C D E F }
    // To be able to extract tuple with 7 arguments
    factory! { A B C D E F G}
    // To be able to extract tuple with 8 arguments
    factory! { A B C D E F G H }
    // To be able to extract tuple with 9 arguments
    factory! { A B C D E F G H I}
    // To be able to extract tuple with 10 arguments
    factory! { A B C D E F G H I J }
    // To be able to extract tuple with 11 arguments
    factory! { A B C D E F G H I J K}
    // To be able to extract tuple with 12 arguments
    factory! { A B C D E F G H I J K L }
    // To be able to extract tuple with 13 arguments
    factory! { A B C D E F G H I J K L M}
    // To be able to extract tuple with 14 arguments
    factory! { A B C D E F G H I J K L M N }
    // To be able to extract tuple with 15 arguments
    factory! { A B C D E F G H I J K L M N O}
    // To be able to extract tuple with 16 arguments
    factory! { A B C D E F G H I J K L M N O P }
    // To be able to extract tuple with 17 arguments
    factory! { A B C D E F G H I J K L M N O P Q}
    // To be able to extract tuple with 18 arguments
    factory! { A B C D E F G H I J K L M N O P Q R }
    // To be able to extract tuple with 19 arguments
    factory! { A B C D E F G H I J K L M N O P Q R S }
    // To be able to extract tuple with 20 arguments
    factory! { A B C D E F G H I J K L M N O P Q R S T }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::{Bot, Reqwest},
        context::Context,
        types::Update,
    };

    #[test]
    fn test_extract() {
        let bot = Arc::new(Bot::<Reqwest>::default());
        let update = Arc::new(Update::default());
        let context = Arc::new(Context::default());

        let _: () =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
        let _: Option<()> =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
        let _: Result<(), Infallible> =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
        let _: Box<()> =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
        let _: Pin<Box<()>> =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
    }
}
