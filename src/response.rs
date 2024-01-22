use std::collections::HashMap;

use crate::{constants, HttpStatusCode};

mod builder;
pub use builder::HttpResponseBuilder;

/// HTTP response
///
/// https://tools.ietf.org/html/rfc2616#section-6
#[derive(Clone, Debug)]
pub struct HttpResponse {
    // TODO?: include HTTP version - idk if it should be included in response tho
    pub(crate) status: HttpStatusCode,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Option<Vec<u8>>,
}

impl HttpResponse {
    /// HTTP-Version Status-Code Reason-Phrase CRLF
    pub fn status_line(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 {} {} {}",
            self.status.code(),
            self.status.phrase(),
            constants::CRLF
        )
        .into_bytes()
    }

    /// (
    ///   key: value CRLF
    /// )+
    /// CRLF
    pub fn headers(&self) -> Vec<u8> {
        let crlf_bytes = constants::CRLF.as_bytes();

        let mut buf = Vec::new();

        // Content length header
        let body_len = self.body.as_ref().map(|body| body.len()).unwrap_or(0);
        buf.extend_from_slice(b"Content-Length: ");
        buf.extend_from_slice(body_len.to_string().as_bytes());
        buf.extend_from_slice(crlf_bytes);

        self.headers
            .iter()
            .filter(|(key, _value)| {
                // TODO: case insensitive
                if key.starts_with("Proxy-") || key.starts_with("Sec-") {
                    return false;
                }

                // TODO: filter forbidden header names
                // https://developer.mozilla.org/en-US/docs/Glossary/Forbidden_header_name

                true
            })
            .for_each(|(key, value)| {
                buf.extend_from_slice(key.as_bytes());
                buf.extend_from_slice(b": ");
                buf.extend_from_slice(value.as_bytes());
                buf.extend_from_slice(crlf_bytes);
            });

        buf.extend_from_slice(crlf_bytes);
        buf
    }

    /// message-body
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }
}

// pub trait ToResponse {
//     fn to_response(self) -> HttpResponse;
// }

// impl<T> ToResponse for T
// where
//     T: Into<String>,
// {
//     fn to_response(self) -> HttpResponse {
//         HttpResponse {
//             status: 200,
//             headers: HashMap::new(),
//             body: Some(self.into()),
//         }
//     }
// }

// impl<T> ToResponse for (u32, T)
// where
//     T: Into<String>,
// {
//     fn to_response(self) -> HttpResponse {
//         todo!()
//     }
// }

// TODO: Json type -> impl From<Json> for HttpResponse

impl<T> From<T> for HttpResponse
where
    T: Into<Vec<u8>>,
{
    fn from(body: T) -> Self {
        Self {
            status: HttpStatusCode::Ok,
            headers: HashMap::new(),
            body: Some(body.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: tests
}
