//! Protocol layer
//!
//! Supported protocols:
//! - HTTP

// TODO http impl

mod http;

pub use http::HttpProtocol;

// TODO?: rename, things like TCP are protocols
pub trait Protocol {
    /// The request type for this protocol (e.g., HttpRequest, WsRequest, etc.)
    type Req;

    /// The response type for this protocol
    type Res;

    /// Convert raw bytes into a strongly-typed request
    fn parse_request(&self, raw: &[u8]) -> Option<Self::Req>;

    /// Convert a strongly-typed response into raw bytes
    fn serialize_response(&self, response: &Self::Res) -> Vec<u8>;
}

// TODO: allow async
pub type Handler<P>
where
    P: Protocol,
= dyn Sync + Send + Fn(P::Req) -> P::Res;
