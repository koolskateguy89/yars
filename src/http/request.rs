use std::collections::HashMap;
use std::sync::Arc;

mod parser;

type Headers = HashMap<String, String>;

// TODO: body content negotiation, e.g. json (serde or whatever - feature flag)

/// HTTP request
///
/// <https://tools.ietf.org/html/rfc2616#section-5>
#[derive(Debug)]
pub struct HttpRequest {
    pub method: RequestMethod,
    pub uri: Arc<str>,
    pub headers: Headers,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
}

impl HttpRequest {
    pub(crate) fn parse_request(buf: &str) -> Option<HttpRequest> {
        let mut lines = buf.lines();

        let first_line = lines.next()?;
        let (method, uri) = parser::parse_request_line(first_line)?;

        let headers = parser::parse_headers(&mut lines);

        // TODO: check this by doing a post req
        // TODO: body
        // TODO?: keep as bytes
        let body: String = lines.collect();
        dbg!(&body);

        Some(HttpRequest {
            method,
            uri: uri.into(),
            headers,
            body: None,
        })
    }

    // TODO: nom parsing
    // TODO: perf testing
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO

    #[test]
    fn test_parse_request() {
        let req = HttpRequest::parse_request("GET / HTTP/1.1\r\n\r\n");
        dbg!(&req);
        assert!(req.is_some());
    }
}
