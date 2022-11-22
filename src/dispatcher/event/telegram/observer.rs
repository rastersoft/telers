use crate::{
    client::Bot,
    context::Context,
    dispatcher::{
        event::{
            bases::{EventReturn, PropagateEventResult},
            service::{Service, ServiceFactory},
            telegram::{Handler, HandlerObject, HandlerObjectService, HandlerRequest},
        },
        middlewares,
    },
    error::app,
    extract::FromEventAndContext,
    filters::Filter,
    types::Update,
};

use futures_core::future::LocalBoxFuture;
use std::{
    cell::RefCell,
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

/// Data for telegram observer service
#[derive(Clone)]
pub struct Request {
    bot: Rc<Bot>,
    update: Rc<Update>,
    context: Rc<RefCell<Context>>,
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.bot, &other.bot)
            && Rc::ptr_eq(&self.update, &other.update)
            && Rc::ptr_eq(&self.context, &other.context)
    }
}

impl Request {
    #[must_use]
    pub fn new(bot: Rc<Bot>, update: Rc<Update>, context: Rc<RefCell<Context>>) -> Self {
        Self {
            bot,
            update,
            context,
        }
    }

    #[must_use]
    pub fn bot(&self) -> Rc<Bot> {
        Rc::clone(&self.bot)
    }

    #[must_use]
    pub fn update(&self) -> Rc<Update> {
        Rc::clone(&self.update)
    }

    #[must_use]
    pub fn context(&self) -> Rc<RefCell<Context>> {
        Rc::clone(&self.context)
    }
}

impl From<Request> for HandlerRequest {
    fn from(req: Request) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

/// Response from telegram observer service
pub struct Response {
    request: Request,
    response: PropagateEventResult,
}

impl Response {
    #[must_use]
    pub fn request(&self) -> &Request {
        &self.request
    }

    #[must_use]
    pub fn response(&self) -> &PropagateEventResult {
        &self.response
    }
}

/// Event observer for Telegram events.
/// Here you can register handler with filter.
/// This observer will stop event propagation when first handler is pass.
pub struct Observer {
    /// Event observer name
    event_name: &'static str,
    /// Handlers of the observer
    handlers: Vec<HandlerObject>,
    /// Common handler of the observer with dummy callback which never will be used
    common_handler: HandlerObject,
    /// Inner middlewares manager
    pub middlewares: middlewares::inner::Manager,
    /// Outer middlewares manager
    pub outer_middlewares: middlewares::outer::Manager,
}

impl Observer {
    /// Create a new event observer
    /// # Arguments
    /// * `event_name` - Event observer name, can be used for logging
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
            common_handler: HandlerObject::new(
                || async move { unimplemented!("This is only for observer filters and without logic") },
                vec![],
            ),
            middlewares: middlewares::inner::Manager::default(),
            outer_middlewares: middlewares::outer::Manager::default(),
        }
    }

    /// Get event observer name
    #[must_use]
    pub fn event_name(&self) -> &str {
        self.event_name
    }

    /// Get handlers of this event observer
    #[must_use]
    pub fn handlers(&self) -> &[HandlerObject] {
        &self.handlers
    }

    /// Get filters for all handlers of this event observer
    #[must_use]
    pub fn filters(&self) -> &[Box<dyn Filter>] {
        self.common_handler.filters()
    }

    /// Register filter for all handlers of this event observer
    /// # Arguments
    /// * `filter` - Filter for the observer
    pub fn filter(&mut self, filter: Box<dyn Filter>) {
        self.common_handler.filter(filter);
    }

    /// Register event handler
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register<H, Args>(&mut self, handler: H, filters: Vec<Box<dyn Filter>>)
    where
        H: Handler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.handlers.push(HandlerObject::new(handler, filters));
    }

    /// Alias to [`Observer::register`] method
    pub fn on<H, Args>(&mut self, handler: H, filters: Vec<Box<dyn Filter>>)
    where
        H: Handler<Args> + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register(handler, filters);
    }
}

impl Debug for Observer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Observer")
            .field("event_name", &self.event_name)
            .finish()
    }
}

impl Default for Observer {
    fn default() -> Self {
        Self::new("default")
    }
}

impl AsRef<Observer> for Observer {
    #[must_use]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ServiceFactory<Request> for Observer {
    type Response = Response;
    type Error = app::Error;
    type Config = ();
    type Service = ObserverService;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    /// Create [`ObserverService`] from [`Observer`]
    fn new_service(&self, _: Self::Config) -> Self::Future {
        let event_name = self.event_name;
        let futs = self
            .handlers
            .iter()
            .map(|handler| handler.new_service(()))
            .collect::<Vec<_>>();
        let fut = self.common_handler.new_service(());
        let middlewares = self.middlewares.middlewares().to_vec();
        let outer_middlewares = self.outer_middlewares.middlewares().to_vec();

        Box::pin(async move {
            let mut handlers = vec![];
            for fut in futs {
                handlers.push(fut.await?);
            }

            let common_handler = fut.await?;

            Ok(ObserverService {
                event_name,
                handlers: Rc::new(handlers),
                common_handler: Rc::new(common_handler),
                middlewares: middlewares.clone(),
                outer_middlewares: outer_middlewares.clone(),
            })
        })
    }
}

/// Service for [`Observer`]
#[allow(clippy::module_name_repetitions)]
pub struct ObserverService {
    /// Event observer name
    event_name: &'static str,
    /// Handler services of the observer
    handlers: Rc<Vec<HandlerObjectService>>,
    /// Common handler service of the observer with dummy callback which never will be used
    common_handler: Rc<HandlerObjectService>,
    /// Inner middlewares
    middlewares: Vec<Rc<Box<dyn middlewares::inner::Middleware>>>,
    /// Outer middlewares
    outer_middlewares: Vec<Rc<Box<dyn middlewares::outer::Middleware>>>,
}

impl ObserverService {
    /// Get event observer name
    #[must_use]
    pub fn event_name(&self) -> &str {
        self.event_name
    }

    /// Get inner middlewares
    #[must_use]
    pub fn middlewares(&self) -> &[Rc<Box<dyn middlewares::inner::Middleware>>] {
        &self.middlewares
    }

    /// Get outer middlewares
    #[must_use]
    pub fn outer_middlewares(&self) -> &[Rc<Box<dyn middlewares::outer::Middleware>>] {
        &self.outer_middlewares
    }

    /// Propagate event to handlers and stops propagation on first match.
    /// Handler will be called when all its filters is pass.
    /// # Errors
    /// If any handler returns error. Probably it's error to extract args to the handler.
    #[allow(clippy::similar_names)]
    pub async fn trigger(&self, req: Request) -> Result<Response, app::Error> {
        let handler_req = req.clone().into();

        // Check observer filters
        if !self.common_handler.check(&handler_req) {
            return Ok(Response {
                request: req,
                response: PropagateEventResult::Rejected,
            });
        }

        for handler in self.handlers.iter() {
            // Check handler filters
            if !handler.check(&handler_req) {
                // If filters isn't pass, skip handler
                continue;
            }

            // If middlewares is empty, we can call handler directly,
            // otherwise we call middlewares with handler, that will be called in any middleware
            let res = if self.middlewares.is_empty() {
                handler.call(handler_req.clone()).await?
            } else {
                // Create middlewares chain
                let middleware = Rc::clone(&self.middlewares[0]);
                let next_middlewares = Box::new(self.middlewares[1..].to_vec().clone().into_iter());

                // Call first middleware (it will call next middlewares or handler)
                middleware
                    .call(handler.service(), handler_req.clone(), next_middlewares)
                    .await?
            };
            // If handler returns skip, we should skip it and run next handler
            if res.response().is_skip() {
                continue;
            }
            // If handler returns cancel, we should stop propagation
            if res.response().is_cancel() {
                return Ok(Response {
                    request: req,
                    response: PropagateEventResult::Rejected,
                });
            }
            // Return a response if it isn't skip or cancel
            return Ok(Response {
                request: req,
                response: PropagateEventResult::Handled(res),
            });
        }

        // Return a response if the event unhandled by observer
        Ok(Response {
            request: req,
            response: PropagateEventResult::Unhandled,
        })
    }
}

impl Debug for ObserverService {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObserverService")
            .field("event_name", &self.event_name)
            .finish()
    }
}

impl Service<Request> for ObserverService {
    type Response = Response;
    type Error = app::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, _: Request) -> Self::Future {
        log::error!("{:?}: Should not be called", self);

        unimplemented!(
            "ObserverService is not intended to be called directly. \
            Use ObserverService::trigger instead"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dispatcher::event::bases::Action,
        filters::{Command, CommandPatternType},
        types::Message,
    };

    macro_rules! r#await {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_observer_trigger() {
        let bot = Rc::new(Bot::default());
        let context = Rc::new(RefCell::new(Context::new()));

        let mut observer = Observer::new("test");
        // Register common filter, which handlers can't pass
        observer.filter(Box::new(Command {
            commands: vec![CommandPatternType::Text("start")],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        }));
        observer.register(|| async {}, vec![]);
        observer.register(
            || async {
                unimplemented!("It's shouldn't trigger because the first handler handles the event")
            },
            vec![],
        );

        let observer_service = r#await!(observer.new_service(())).unwrap();
        let req = Request {
            bot: Rc::clone(&bot),
            update: Rc::new(Update::default()),
            context: Rc::clone(&context),
        };
        let res = r#await!(observer_service.trigger(req.clone())).unwrap();

        // Filter not pass, so handler should be rejected
        match res.response() {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }

        let req = Request {
            bot: req.bot(),
            update: Rc::new(Update {
                message: Some(Message {
                    text: Some("/start".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            context: req.context(),
        };
        let res = r#await!(observer_service.trigger(req)).unwrap();

        // Filter pass, so handler should be handled
        match res.response() {
            PropagateEventResult::Handled(_) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_observer_event_return() {
        let bot = Rc::new(Bot::default());
        let context = Rc::new(RefCell::new(Context::new()));
        let update = Rc::new(Update::default());

        let mut observer = Observer::new("test");
        observer.register(|| async { Action::Skip }, vec![]);
        observer.register(|| async {}, vec![]);

        let observer_service = r#await!(observer.new_service(())).unwrap();

        let req = Request {
            bot: Rc::clone(&bot),
            update: Rc::clone(&update),
            context: Rc::clone(&context),
        };
        let res = r#await!(observer_service.trigger(req.clone())).unwrap();

        // First handler returns `Action::Skip`, so second handler should be called
        match res.response() {
            PropagateEventResult::Handled(handler_res) => {
                assert_eq!(*handler_res.response(), EventReturn::default());
            }
            _ => panic!("Unexpected result"),
        }

        let mut observer = Observer::new("test2");
        observer.register(|| async { Action::Skip }, vec![]);
        observer.register(|| async { Action::Cancel }, vec![]);

        let observer_service = r#await!(observer.new_service(())).unwrap();

        let res = r#await!(observer_service.trigger(req.clone())).unwrap();

        // First handler returns `Action::Skip`, so second handler should be called and it returns `Action::Cancel`,
        // so response should be `PropagateEventResult::Rejected`
        match res.response() {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }
    }
}
