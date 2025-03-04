//! Protocol layer
//!
//! Supported protocols:
//! - HTTP

mod http;

pub use http::HttpProtocol;

// TODO?: rename, things like TCP are protocols. maybe Codec?
/// Message/communication protocol layer.
///
/// Responsible for converting raw bytes into higher-level request/response objects.
pub trait Protocol {
    /// The request type for this protocol (e.g., HttpRequest, WsRequest, etc.)
    type Req;

    /// The response type for this protocol
    type Res;

    /// The routing key type for this protocol
    // TODO?: change Debug to Display
    type RoutingKey: Eq + std::hash::Hash + std::fmt::Debug;

    // TODO: change to result, or maybe result<option>, idk
    /// Convert raw bytes into a strongly-typed request
    fn parse_request(&self, raw: &[u8]) -> Option<Self::Req>;

    /// Convert a strongly-typed response into raw bytes
    fn serialize_response(&self, response: &Self::Res) -> Vec<u8>;

    /// Extract a routing key from a request.
    fn extract_routing_key(&self, req: &Self::Req) -> Self::RoutingKey;
}

// TODO: allow async
pub type Handler<P>
where
    P: Protocol,
= dyn Sync + Send + Fn(P::Req) -> P::Res;

pub trait ToHandler<P>
where
    P: Protocol,
{
    fn to_handler(self) -> Box<Handler<P>>;
}

impl<P, F, B> ToHandler<P> for F
where
    P: Protocol,
    F: Sync + Send + Fn(P::Req) -> B + 'static,
    B: Into<P::Res>,
{
    fn to_handler(self) -> Box<Handler<P>> {
        Box::new(move |req| self(req).into())
    }
}
