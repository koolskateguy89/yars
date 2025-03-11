use std::collections::HashMap;

mod parser;

// TODO?: headers should be map<string, vec<string>>
type Headers = HashMap<String, String>;

/// HTTP request
///
/// <https://tools.ietf.org/html/rfc2616#section-5>
#[derive(Debug)]
pub struct HttpRequest {
    pub method: RequestMethod,
    pub uri: String,
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
        parser::parse_request_nom(buf).map(|(_input, req)| req).ok()
    }
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
