//! https://www.w3.org/Protocols/HTTP/1.0/spec.html

use std::str::Lines;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    combinator::map,
    multi::many0,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

use super::{Headers, HttpRequest, RequestMethod};
use crate::constants::CRLF;

const HTTP_1_1: &str = "HTTP/1.1";

/// Method Request-URI HTTP-Version
fn old_parse_request_line(status_line: &str) -> Option<(RequestMethod, &str)> {
    dbg!(status_line);
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
/// NAME: VALUE
fn old_parse_headers(lines: &mut Lines) -> Headers {
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

pub(crate) fn parse_request(buf: &str) -> Option<HttpRequest> {
    let mut lines = buf.lines();

    let first_line = lines.next()?;
    let (method, uri) = old_parse_request_line(first_line)?;

    let headers = old_parse_headers(&mut lines);

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

// nom

fn http_method(input: &str) -> IResult<&str, RequestMethod> {
    alt((
        map(tag("GET"), |_| RequestMethod::GET),
        map(tag("POST"), |_| RequestMethod::POST),
        map(tag("PUT"), |_| RequestMethod::PUT),
        map(tag("DELETE"), |_| RequestMethod::DELETE),
        map(tag("HEAD"), |_| RequestMethod::HEAD),
        map(tag("OPTIONS"), |_| RequestMethod::OPTIONS),
        map(tag("CONNECT"), |_| RequestMethod::CONNECT),
        map(tag("TRACE"), |_| RequestMethod::TRACE),
        map(tag("PATCH"), |_| RequestMethod::PATCH),
    ))
    .parse(input)
}

/// Method Request-URI HTTP-Version CRLF
fn request_line(input: &str) -> IResult<&str, (RequestMethod, &str)> {
    map(
        (
            http_method,
            tag(" "),
            take_until1(" "), // uri
            tag(" "),
            tag(HTTP_1_1), // http version
            tag(CRLF),
        ),
        |(method, _, uri, _, _, _)| (method, uri),
    )
    .parse(input)
}

/// NAME: VALUE CRLF
fn header(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(
        separated_pair(
            take_until1(": "), // header name
            tag(": "),
            take_until1(CRLF), // header value
        ),
        tag(CRLF),
    )
    .parse(input)
}

/// Trailing newline after headers
fn headers(input: &str) -> IResult<&str, Headers> {
    map(terminated(many0(header), tag(CRLF)), |header_list| {
        // Need to clone into String
        header_list
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    })
    .parse(input)
}

pub(super) fn parse_request_nom(input: &str) -> IResult<&str, HttpRequest> {
    let (input, (method, uri)) = request_line(input)?;
    let (input, headers) = headers(input)?;

    // TODO?: body should be rest of input
    Ok((
        input,
        HttpRequest {
            method,
            uri: uri.to_string(),
            headers,
            body: None,
        },
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_parse_request_line() {
        let (method, uri) = old_parse_request_line("GET / HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::GET);
        assert_eq!(uri, "/");

        let (method, uri) = old_parse_request_line("POST /abc HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::POST);
        assert_eq!(uri, "/abc");

        let (method, uri) = old_parse_request_line("PUT / HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::PUT);
        assert_eq!(uri, "/");

        let (method, uri) = old_parse_request_line("DELETE / HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::DELETE);
        assert_eq!(uri, "/");

        let (method, uri) = old_parse_request_line("HEAD / HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::HEAD);
        assert_eq!(uri, "/");

        let (method, uri) = old_parse_request_line("OPTIONS / HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::OPTIONS);
        assert_eq!(uri, "/");

        let (method, uri) = old_parse_request_line("CONNECT / HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::CONNECT);
        assert_eq!(uri, "/");

        let (method, uri) = old_parse_request_line("TRACE / HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::TRACE);
        assert_eq!(uri, "/");

        let (method, uri) = old_parse_request_line("PATCH / HTTP/1.1").unwrap();
        assert_eq!(method, RequestMethod::PATCH);
        assert_eq!(uri, "/");
    }

    #[test]
    fn test_parse_headers() {
        let mut lines = "Host: localhost:8080\r\n\
        User-Agent: curl/7.68.0\r\n\
        Accept: */*\r\n\
        \r\n"
            .lines();

        let headers = old_parse_headers(&mut lines);
        assert_eq!(headers.len(), 3);
        assert_eq!(headers.get("Host").unwrap(), "localhost:8080");
        assert_eq!(headers.get("User-Agent").unwrap(), "curl/7.68.0");
        assert_eq!(headers.get("Accept").unwrap(), "*/*");
    }

    // nom

    #[test]
    fn parse_http_method() {
        assert_eq!(http_method("GET"), Ok(("", RequestMethod::GET)));
        assert_eq!(http_method("POST"), Ok(("", RequestMethod::POST)));
        assert_eq!(http_method("PUT"), Ok(("", RequestMethod::PUT)));
        assert_eq!(http_method("DELETE"), Ok(("", RequestMethod::DELETE)));
        assert_eq!(http_method("HEAD"), Ok(("", RequestMethod::HEAD)));
        assert_eq!(http_method("OPTIONS"), Ok(("", RequestMethod::OPTIONS)));
        assert_eq!(http_method("CONNECT"), Ok(("", RequestMethod::CONNECT)));
        assert_eq!(http_method("TRACE"), Ok(("", RequestMethod::TRACE)));
        assert_eq!(http_method("PATCH"), Ok(("", RequestMethod::PATCH)));
    }

    #[test]
    fn parse_request_line() {
        assert_eq!(
            request_line("GET / HTTP/1.1\r\n"),
            Ok(("", (RequestMethod::GET, "/")))
        );

        assert_eq!(
            request_line("POST /abc/okay/okay HTTP/1.1\r\n"),
            Ok(("", (RequestMethod::POST, "/abc/okay/okay")))
        );
    }

    #[test]
    fn request_line_doesnt_accept_incorrect_http_version() {
        assert!(request_line("GET / HTTP/1.0\r\n").is_err());
        assert!(request_line("GET / HTTP/2.0\r\n").is_err());
    }

    #[test]
    fn parse_header() {
        assert_eq!(
            header("Host: localhost:8080\r\n"),
            Ok(("", ("Host", "localhost:8080")))
        );
        assert_eq!(
            header("User-Agent: curl/7.68.0\r\n"),
            Ok(("", ("User-Agent", "curl/7.68.0")))
        );

        // Should keep rest of input
        assert_eq!(
            header("Accept: */*\r\nRest"),
            Ok(("Rest", ("Accept", "*/*")))
        );
    }

    #[test]
    fn header_doesnt_accept_missing_newline() {
        assert!(header("User-Agent: curl/7.68.0").is_err());
    }

    #[test]
    fn parse_headers() {
        let input = "Host: localhost:8080\r\n\
        User-Agent: curl/7.68.0\r\n\
        Accept: */*\r\n\
        \r\n";

        assert_eq!(
            headers(input),
            Ok((
                "",
                HashMap::from([
                    ("Host".to_string(), "localhost:8080".to_string()),
                    ("User-Agent".to_string(), "curl/7.68.0".to_string()),
                    ("Accept".to_string(), "*/*".to_string()),
                ])
            ))
        );
    }

    #[test]
    fn parse_empty_headers() {
        assert_eq!(headers("\r\n"), Ok(("", HashMap::new())));
    }

    #[test]
    fn headers_doesnt_accept_missing_trailing_newline() {
        assert!(headers("User-Agent: curl/7.68.0\r\n").is_err());
    }
}
