use std::{collections::HashMap, str::Lines};

use nom::*;

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

// TODO: extract parse_ funcs into parser submodule

/// Method Request-URI HTTP-Version
fn parse_status_line(status_line: &str) -> Option<(RequestMethod, &str)> {
    let mut status_line = status_line.split_whitespace();

    let method = status_line.next()?;
    let uri = status_line.next()?;
    let _http_version = status_line.next()?;

    let method = match method {
        "GET" => RequestMethod::GET,
        "POST" => RequestMethod::POST,
        "PUT" => RequestMethod::PUT,
        "DELETE" => RequestMethod::DELETE,
        "HEAD" => RequestMethod::HEAD,
        "OPTIONS" => RequestMethod::OPTIONS,
        "CONNECT" => RequestMethod::CONNECT,
        "TRACE" => RequestMethod::TRACE,
        "PATCH" => RequestMethod::PATCH,
        _ => return None,
    };

    Some((method, uri))
}

/// headers
/// KEY: VALUE
fn parse_headers(lines: &mut Lines) -> Headers {
    let mut lines = lines.peekable();

    let mut headers = Headers::new();

    while let Some(line) = lines.peek() {
        // End of headers
        if line.is_empty() {
            break;
        }

        let line = lines.next().unwrap();
        let mut line = line.splitn(2, ": ");

        let key = match line.next() {
            Some(key) => key.to_string(),
            None => continue,
        };

        let value = match line.next() {
            Some(value) => value.to_string(),
            None => continue,
        };

        headers.insert(key, value);
    }

    headers
}

impl HttpRequest {
    pub(crate) fn parse_request(buf: String) -> Option<HttpRequest> {
        // TODO: use nom
        let mut lines = buf.lines();

        let first_line = lines.next()?;
        let (method, uri) = parse_status_line(first_line)?;

        let headers = parse_headers(&mut lines);

        // TODO: body

        Some(HttpRequest {
            method,
            uri: uri.to_string(),
            headers,
            body: None,
        })
    }
}
