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
        f.debug_struct("Router")
            .field("routes", &self.routes.keys())
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

    pub(crate) fn add_route<H>(&mut self, routing_key: P::RoutingKey, handler: H)
    where
        H: ToHandler<P>,
    {
        self.routes.insert(routing_key, handler.to_handler());
    }

    pub(crate) fn set_default_handler<H>(&mut self, handler: H)
    where
        H: ToHandler<P>,
    {
        self.default_handler = Some(handler.to_handler());
    }

    pub(crate) fn get_request_handler(&self, routing_key: P::RoutingKey) -> Option<&Handler<P>> {
        let boxed_handler = self
            .routes
            .get(&routing_key)
            .or(self.default_handler.as_ref());

        // Extract a reference to the handler from the Box
        boxed_handler.map(AsRef::as_ref)
    }
}
