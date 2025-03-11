use std::collections::HashMap;

use crate::protocol::{Handler, Protocol, ToHandler};

pub(crate) struct Router<P>
where
    P: Protocol,
{
    routes: HashMap<P::RoutingKey, Box<Handler<P>>>,
    default_handler: Option<Box<Handler<P>>>,
}

impl<P> std::fmt::Debug for Router<P>
where
    P: Protocol,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys = self
            .routes
            .keys()
            .map(|k| format!("{}", k))
            .collect::<Vec<_>>();

        f.debug_struct("Router")
            .field("routes", &keys)
            .field("default_handler", &self.default_handler.is_some())
            .finish()
    }
}

impl<P> Router<P>
where
    P: Protocol,
{
    pub(crate) fn new() -> Self {
        Self {
            routes: HashMap::new(),
            default_handler: None,
        }
    }

    /// Adds a route with key [`routing_key`][Protocol::RoutingKey] that will call the given
    /// [`handler`][Handler]
    pub(crate) fn add_route(&mut self, routing_key: P::RoutingKey, handler: impl ToHandler<P>) {
        self.routes.insert(routing_key, handler.to_handler());
    }

    /// Sets the default handler. It will be returned in
    /// [`get_request_handler`][Self::get_request_handler] if no route matches
    pub(crate) fn set_default_handler(&mut self, handler: impl ToHandler<P>) {
        self.default_handler = Some(handler.to_handler());
    }

    /// Gets the handler for the given [`routing_key`][Protocol::RoutingKey], according to handlers
    /// previously added with [`add_route`][Self::add_route]. If no handler is found, returns the
    /// default handler, if set.
    pub(crate) fn get_request_handler(&self, routing_key: &P::RoutingKey) -> Option<&Handler<P>> {
        let maybe_boxed_handler = self
            .routes
            .get(routing_key)
            .or(self.default_handler.as_ref());

        // Extract a reference to the handler from the Box
        maybe_boxed_handler.map(|boxed_handler| boxed_handler.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        http::RequestMethod,
        protocol::{http::HttpRoutingKey, HttpProtocol, Protocol},
    };

    const KEY: (&str, RequestMethod) = ("/", RequestMethod::GET);

    #[test]
    fn can_add_route() {
        let mut router = Router::<HttpProtocol>::new();
        let key: HttpRoutingKey = KEY.into();

        assert!(router.get_request_handler(&key).is_none());

        let handler = |_req| -> crate::Result<<HttpProtocol as Protocol>::Res> { unimplemented!() };
        router.add_route(key.clone(), handler);

        assert!(router.get_request_handler(&key).is_some());
    }

    #[test]
    fn can_set_default_handler() {
        let mut router = Router::<HttpProtocol>::new();
        let key: HttpRoutingKey = KEY.into();

        assert!(router.get_request_handler(&key).is_none());

        let default_handler =
            |_req| -> crate::Result<<HttpProtocol as Protocol>::Res> { unimplemented!() };
        router.set_default_handler(default_handler);

        assert!(router.get_request_handler(&key).is_some());
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Router<HttpProtocol>>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Router<HttpProtocol>>();
    }
}
