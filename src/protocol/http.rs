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
#[derive(PartialEq, Eq, Hash, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routing_key_displays_correctly() {
        let key = HttpRoutingKey {
            uri: "/route".to_string(),
            method: RequestMethod::GET,
        };

        assert_eq!(key.to_string(), "GET /route");
    }

    #[test]
    fn test_http_protocol() {
        let protocol = HttpProtocol;

        let raw = b"GET / HTTP/1.1\r\n\r\n".to_vec();
        let req = protocol.parse_request(raw).unwrap();
        assert_eq!(req.method, RequestMethod::GET);
        assert_eq!(req.uri, "/");

        let routing_key = protocol.extract_routing_key(&req);
        assert_eq!(routing_key.uri, "/");
        assert_eq!(routing_key.method, RequestMethod::GET);
    }
}
