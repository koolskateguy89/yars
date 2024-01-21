use std::{collections::HashMap, fmt::Display};

use crate::{constants, HttpStatusCode};

/// HTTP response
///
/// https://tools.ietf.org/html/rfc2616#section-6
#[derive(Clone, Debug)]
pub struct HttpResponse {
    // TODO?: include HTTP version - idk if it should be included in response tho
    pub(crate) status: HttpStatusCode,
    pub(crate) headers: HashMap<String, String>,
    // TODO?: change to bytes?
    pub(crate) body: Option<String>,
}

// TODO?: make a builder for HttpResponse
impl HttpResponse {
    pub fn status(mut self, status: HttpStatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn headers<T>(mut self, headers: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.headers = headers.into_iter().collect();
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn json(self, json: impl Into<String>) -> Self {
        self.header("Content-Type", "application/json").body(json)
    }

    pub fn html(self, html: impl Into<String>) -> Self {
        self.header("Content-Type", "text/html").body(html)
    }

    // TODO: .xml(xml: String) (final)
    // TODO: .text(text: String) (final)

    // TODO?: some way to send binary data
}

impl Display for HttpResponse {
    /// Response:
    /// HTTP-Version Status-Code Reason-Phrase CRLF
    /// headers CRLF
    /// message-body
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Status line
        write!(
            f,
            "HTTP/1.1 {} {} {}",
            self.status.code(),
            self.status.phrase(),
            constants::CRLF
        )?;

        // Content length header
        let body_len = self.body.as_ref().map(|body| body.len()).unwrap_or(0);
        write!(f, "Content-Length: {body_len}{}", constants::CRLF)?;

        self.headers
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .try_for_each(|header| write!(f, "{header}{}", constants::CRLF))?;

        write!(f, "{}", constants::CRLF)?;

        if let Some(ref body) = self.body {
            write!(f, "{body}{}", constants::CRLF)?;
        }

        Ok(())
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
    T: Into<String>,
{
    fn from(body: T) -> Self {
        Self {
            status: HttpStatusCode::Ok,
            headers: HashMap::new(),
            body: Some(body.into()),
        }
    }
}

// TODO: tests
