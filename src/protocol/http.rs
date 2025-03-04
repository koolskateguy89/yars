use super::Protocol;
// TODO: move these to be under some /http
use crate::{request::RequestMethod, HttpRequest, HttpResponse};

/// HTTP 1.1
pub struct HttpProtocol;

impl Protocol for HttpProtocol {
    type Req = HttpRequest;

    type Res = HttpResponse;

    // FIXME?: i dont like the debug display for this - I want [method] [uri]
    // but it shows ("uri", method)
    type RoutingKey = (String, RequestMethod);

    fn parse_request(&self, raw: &[u8]) -> Option<Self::Req> {
        // TODO: not make a String, instead just parse from bytes
        let raw_string = String::from_utf8(raw.to_vec()).ok()?;

        HttpRequest::parse_request(raw_string)
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
        (req.uri.clone(), req.method)
    }
}
