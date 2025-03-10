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
pub trait Protocol: Send + Sync + 'static {
    /// The request type for this protocol (e.g., HttpRequest, WsRequest, etc.)
    type Req: Send + Sync;

    /// The response type for this protocol
    type Res: Send + Sync;

    /// The routing key type for this protocol
    type RoutingKey: Send + Sync + Eq + std::hash::Hash + std::fmt::Display;

    // TODO: change to result, or maybe result<option>, idk
    /// Convert raw bytes into a strongly-typed request
    fn parse_request(&self, raw: Vec<u8>) -> Option<Self::Req>;

    /// Convert a strongly-typed response into raw bytes
    fn serialize_response(&self, response: &Self::Res) -> Vec<u8>;

    /// Extract a routing key from a request.
    fn extract_routing_key(&self, req: &Self::Req) -> Self::RoutingKey;
}

// TODO: allow async
/// A handler is a function that takes a request and returns a response.
///
/// Can return any generic error
pub type Handler<P> = dyn Sync
    + Send
    + Fn(
        <P as Protocol>::Req,
    ) -> std::result::Result<<P as Protocol>::Res, Box<dyn std::error::Error + Send + Sync>>;

pub trait ToHandler<P>
where
    P: Protocol,
{
    fn to_handler(self) -> Box<Handler<P>>;
}

impl<P, F, R, E> ToHandler<P> for F
where
    P: Protocol,
    F: Sync + Send + Fn(P::Req) -> std::result::Result<R, E> + 'static,
    R: Into<P::Res>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn to_handler(self) -> Box<Handler<P>> {
        Box::new(move |req| self(req).map(|res| res.into()).map_err(|e| e.into()))
    }
}
