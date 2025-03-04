use std::{collections::HashMap, io::BufRead};
use tokio::io::AsyncBufReadExt;

mod parser;

type Headers = HashMap<String, String>;

// TODO: body content negotiation, e.g. json (serde or whatever - feature flag)

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
    pub(crate) fn parse_request(buf: String) -> Option<HttpRequest> {
        let mut lines = buf.lines();

        let first_line = lines.next()?;
        let (method, uri) = parser::parse_status_line(first_line)?;

        let headers = parser::parse_headers(&mut lines);

        // TODO: check this by doing a post req
        // TODO: body
        // TODO?: keep as bytes
        let body: String = lines.collect();
        dbg!(&body);

        Some(HttpRequest {
            method,
            uri: uri.to_string(),
            headers,
            body: None,
        })
    }

    pub(crate) fn parse_request_bytes(mut buf: Vec<u8>) -> Option<HttpRequest> {
        // TODO
        // TODO: figure out correct buf param type & how tf to read it lol
        // let mut lines = tokio::io::AsyncBufReadExt::lines(buf);
        // buf.reader()

        todo!()
    }
}
