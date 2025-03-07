//! Protocol layer
//!
//! Supported protocols:
//! - HTTP

mod http;

use std::panic::{RefUnwindSafe, UnwindSafe};

pub use http::HttpProtocol;

// TODO?: rename, things like TCP are protocols. maybe Codec?
/// Message/communication protocol layer.
///
/// Responsible for converting raw bytes into higher-level request/response objects.
pub trait Protocol: Send + Sync + 'static {
    /// The request type for this protocol (e.g., HttpRequest, WsRequest, etc.)
    type Req: Send + Sync + UnwindSafe;

    /// The response type for this protocol
    type Res: Send + Sync;

    /// The routing key type for this protocol
    type RoutingKey: Send + Sync + Eq + std::hash::Hash + std::fmt::Display;

    // TODO: change to result, or maybe result<option>, idk
    /// Convert raw bytes into a strongly-typed request
    fn parse_request(&self, raw: &[u8]) -> Option<Self::Req>;

    /// Convert a strongly-typed response into raw bytes
    fn serialize_response(&self, response: &Self::Res) -> Vec<u8>;

    /// Extract a routing key from a request.
    fn extract_routing_key(&self, req: &Self::Req) -> Self::RoutingKey;
}

// TODO: result
// TODO: allow async
pub type Handler<P> =
    dyn Sync + Send + RefUnwindSafe + Fn(<P as Protocol>::Req) -> <P as Protocol>::Res;

pub trait ToHandler<P>
where
    P: Protocol,
{
    fn to_handler(self) -> Box<Handler<P>>;
}

impl<P, F, B> ToHandler<P> for F
where
    P: Protocol,
    F: Sync + Send + RefUnwindSafe + Fn(P::Req) -> B + 'static,
    B: Into<P::Res>,
{
    fn to_handler(self) -> Box<Handler<P>> {
        Box::new(move |req| self(req).into())
    }
}
