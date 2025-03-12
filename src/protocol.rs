//! Protocol layer
//!
//! Supported protocols:
//! - HTTP

pub(crate) mod http;

use std::future::Future;
use std::pin::Pin;

pub use http::HttpProtocol;

// TODO?: rename, things like TCP are protocols. maybe Codec?
// TODO?: make these functions async?
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

    /// Convert raw bytes into a strongly-typed request
    fn parse_request(&self, raw: Vec<u8>) -> Option<Self::Req>;

    /// Convert a strongly-typed response into raw bytes
    fn serialize_response(&self, response: &Self::Res) -> Vec<u8>;

    /// Extract a routing key from a request.
    fn extract_routing_key(&self, req: &Self::Req) -> Self::RoutingKey;
}

type AnyError = Box<dyn std::error::Error + Send + Sync>;

type HandlerFuture<P> = dyn Send + Sync + Future<Output = Result<<P as Protocol>::Res, AnyError>>;

/// A handler is an async function that takes a request and returns a response.
pub(crate) type Handler<P> =
    dyn Send + Sync + Fn(<P as Protocol>::Req) -> Pin<Box<HandlerFuture<P>>>;

pub trait ToHandler<P>
where
    P: Protocol,
{
    fn to_handler(self) -> Box<Handler<P>>;
}

impl<P, F, Fut, Res, Err> ToHandler<P> for F
where
    P: Protocol,
    F: Send + Sync + Fn(P::Req) -> Fut + 'static,
    Fut: Send + Sync + Future<Output = Result<Res, Err>> + 'static,
    Res: Into<P::Res>,
    Err: Into<AnyError>,
{
    fn to_handler(self) -> Box<Handler<P>> {
        Box::new(move |req| {
            let handler_fut = self(req);
            let handler_fut = async { handler_fut.await.map(Into::into).map_err(Into::into) };
            Box::pin(handler_fut)
        })
    }
}
