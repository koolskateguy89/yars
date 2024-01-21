use std::collections::HashMap;

mod parser;

type Headers = HashMap<String, String>;

#[derive(Debug)]
pub struct HttpRequest {
    // TODO
    pub method: RequestMethod,
    pub uri: String, // TODO?: diff type
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
    pub(crate) fn parse_request(buf: String) -> Option<HttpRequest> {
        let mut lines = buf.lines();

        let first_line = lines.next()?;
        let (method, uri) = parser::parse_status_line(first_line)?;

        let headers = parser::parse_headers(&mut lines);

        // TODO: body

        Some(HttpRequest {
            method,
            uri: uri.to_string(),
            headers,
            body: None,
        })
    }
}
