use std::sync::Arc;

use super::Protocol;
use crate::http::{HttpRequest, HttpResponse, RequestMethod};

/// HTTP 1.1
pub struct HttpProtocol;

impl Protocol for HttpProtocol {
    type Req = HttpRequest;

    type Res = HttpResponse;

    type RoutingKey = HttpRoutingKey;

    fn parse_request(&self, raw: Vec<u8>) -> Option<Self::Req> {
        let utf8_str = String::from_utf8(raw).ok()?;

        HttpRequest::parse_request(&utf8_str)
    }

    fn serialize_response(&self, response: &Self::Res) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.append(&mut response.status_line());
        buf.append(&mut response.headers());
        if let Some(body) = response.body() {
            buf.extend_from_slice(body);
        }

        buf
    }

    fn extract_routing_key(&self, req: &Self::Req) -> Self::RoutingKey {
        HttpRoutingKey {
            uri: req.uri.clone(),
            method: req.method,
        }
    }
}

/// HTTP routing is based on the URI and the request method
#[derive(PartialEq, Eq, Hash)]
pub struct HttpRoutingKey {
    uri: String,
    method: RequestMethod,
}

impl std::fmt::Display for HttpRoutingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.method, self.uri)
    }
}

impl<T> From<(T, RequestMethod)> for HttpRoutingKey
where
    T: ToString,
{
    fn from((uri, method): (T, RequestMethod)) -> Self {
        Self {
            uri: uri.to_string(),
            method,
        }
    }
}
