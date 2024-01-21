use std::str::Lines;

// TODO: use nom
// use nom::*;

use super::{Headers, RequestMethod};

/// Method Request-URI HTTP-Version
pub(super) fn parse_status_line(status_line: &str) -> Option<(RequestMethod, &str)> {
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
pub(super) fn parse_headers(lines: &mut Lines) -> Headers {
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
