//! Uses the [`nom`] parser combinator library to parse HTTP requests.
//!
//! <https://www.w3.org/Protocols/HTTP/1.0/spec.html>

use std::str;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    combinator::{map, map_res},
    multi::many0,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

use super::{Headers, HttpRequest, RequestMethod};
use crate::constants::CRLF;

// todo?: use newline combinator instead of CRLF

fn http_method(input: &[u8]) -> IResult<&[u8], RequestMethod> {
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

fn http_version(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((tag("HTTP/1.0"), tag("HTTP/1.1"))).parse(input)
}

/// Method Request-URI HTTP-Version CRLF
fn request_line(input: &[u8]) -> IResult<&[u8], (RequestMethod, &str)> {
    map(
        (
            http_method,
            tag(" "),
            map_res(take_until1(" "), str::from_utf8), // uri
            tag(" "),
            http_version,
            tag(CRLF),
        ),
        |(method, _, uri, _, _, _)| (method, uri),
    )
    .parse(input)
}

/// NAME: VALUE CRLF
fn header(input: &[u8]) -> IResult<&[u8], (&str, &str)> {
    terminated(
        separated_pair(
            map_res(take_until1(": "), str::from_utf8), // header name
            tag(": "),
            map_res(take_until1(CRLF), str::from_utf8), // header value
        ),
        tag(CRLF),
    )
    .parse(input)
}

/// Trailing newline after headers
/// <https://www.w3.org/Protocols/HTTP/1.0/spec.html#Message-Headers>
fn headers(input: &[u8]) -> IResult<&[u8], Headers> {
    map(terminated(many0(header), tag(CRLF)), |header_list| {
        // Need to clone into String
        header_list
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    })
    .parse(input)
}

/// <https://www.w3.org/Protocols/HTTP/1.0/spec.html>
pub(super) fn parse_request(input: &[u8]) -> IResult<&[u8], HttpRequest> {
    let (input, (method, uri)) = request_line(input)?;
    let (input, headers) = headers(input)?;

    // Body is rest of input
    let body = if input.is_empty() {
        None
    } else {
        // TODO: if possible, not copy, just take from original input because we _could_ own the input
        Some(input.to_vec())
    };

    Ok((
        b"",
        HttpRequest {
            method,
            uri: uri.to_string(),
            headers,
            body,
        },
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    // to avoid having to write `&b""[..]` everywhere
    const EMPTY: &[u8] = b"";

    #[test]
    fn parse_http_method() {
        assert_eq!(http_method(b"GET"), Ok((EMPTY, RequestMethod::GET)));
        assert_eq!(http_method(b"POST"), Ok((EMPTY, RequestMethod::POST)));
        assert_eq!(http_method(b"PUT"), Ok((EMPTY, RequestMethod::PUT)));
        assert_eq!(http_method(b"DELETE"), Ok((EMPTY, RequestMethod::DELETE)));
        assert_eq!(http_method(b"HEAD"), Ok((EMPTY, RequestMethod::HEAD)));
        assert_eq!(http_method(b"OPTIONS"), Ok((EMPTY, RequestMethod::OPTIONS)));
        assert_eq!(http_method(b"CONNECT"), Ok((EMPTY, RequestMethod::CONNECT)));
        assert_eq!(http_method(b"TRACE"), Ok((EMPTY, RequestMethod::TRACE)));
        assert_eq!(http_method(b"PATCH"), Ok((EMPTY, RequestMethod::PATCH)));
    }

    #[test]
    fn parse_request_line() {
        assert_eq!(
            request_line(b"GET / HTTP/1.0\r\n"),
            Ok((EMPTY, (RequestMethod::GET, "/")))
        );

        assert_eq!(
            request_line(b"GET /test-test HTTP/1.1\r\n"),
            Ok((EMPTY, (RequestMethod::GET, "/test-test")))
        );

        assert_eq!(
            request_line(b"POST /abc/okay/okay HTTP/1.1\r\n"),
            Ok((EMPTY, (RequestMethod::POST, "/abc/okay/okay")))
        );
    }

    #[test]
    fn request_line_doesnt_accept_incorrect_http_version() {
        assert!(request_line(b"GET / HTTP/2.0\r\n").is_err());
    }

    #[test]
    fn parse_header() {
        assert_eq!(
            header(b"Host: localhost:8080\r\n"),
            Ok((EMPTY, ("Host", "localhost:8080")))
        );
        assert_eq!(
            header(b"User-Agent: curl/7.68.0\r\n"),
            Ok((EMPTY, ("User-Agent", "curl/7.68.0")))
        );

        // Should keep rest of input
        assert_eq!(
            header(b"Accept: */*\r\nRest"),
            Ok((&b"Rest"[..], ("Accept", "*/*")))
        );
    }

    #[test]
    fn header_doesnt_accept_missing_newline() {
        assert!(header(b"User-Agent: curl/7.68.0").is_err());
    }

    #[test]
    fn parse_headers() {
        let input = b"Host: localhost:8080\r\n\
        User-Agent: curl/7.68.0\r\n\
        Accept: */*\r\n\
        \r\n";

        assert_eq!(
            headers(input),
            Ok((
                EMPTY,
                HashMap::from([
                    ("Host".to_string(), "localhost:8080".to_string()),
                    ("User-Agent".to_string(), "curl/7.68.0".to_string()),
                    ("Accept".to_string(), "*/*".to_string()),
                ])
            ))
        );
    }

    #[test]
    fn parses_empty_headers() {
        assert_eq!(headers(b"\r\n"), Ok((EMPTY, HashMap::new())));
    }

    #[test]
    fn headers_doesnt_accept_missing_trailing_newline() {
        assert!(headers(b"User-Agent: curl/7.68.0\r\n").is_err());
    }

    #[test]
    fn parses_body() {
        let input = b"GET / HTTP/1.1\r\n\
        \r\n\
        This is the body";

        let (input, req) = parse_request(input).unwrap();
        assert_eq!(input, b"");
        assert_eq!(req.body, Some(b"This is the body".to_vec()));
    }

    #[test]
    fn parses_request() {
        let req = parse_request(b"GET / HTTP/1.1\r\n\
        Host: localhost:8080\r\n\
        \r\n\
        The body\r\n\
        is over multiple lines");
        dbg!(&req);
        assert!(req.is_ok());

        let (input, req) = req.unwrap();
        assert!(input.is_empty());

        assert_eq!(req.method, RequestMethod::GET);
        assert_eq!(req.uri, "/");

        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.headers.get("Host"), Some(&"localhost:8080".to_string()));

        assert_eq!(req.body, Some(b"The body\r\nis over multiple lines".to_vec()));
    }
}
