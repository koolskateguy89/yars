use std::collections::HashMap;

use super::HttpResponse;
use crate::HttpStatusCode;

pub struct HttpResponseBuilder {
    // TODO?: include HTTP version - idk if it should be included in response tho
    pub(crate) status: HttpStatusCode,
    pub(crate) headers: HashMap<String, String>,
}

impl HttpResponseBuilder {
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

    /// Empty body
    pub fn finish(self) -> HttpResponse {
        HttpResponse {
            status: self.status,
            headers: self.headers,
            body: None,
        }
    }

    pub fn body(self, body: impl Into<Vec<u8>>) -> HttpResponse {
        HttpResponse {
            status: self.status,
            headers: self.headers,
            body: Some(body.into()),
        }
    }

    pub fn json(self, json: impl Into<Vec<u8>>) -> HttpResponse {
        self.header("Content-Type", "application/json").body(json)
    }

    pub fn html(self, html: impl Into<Vec<u8>>) -> HttpResponse {
        self.header("Content-Type", "text/html").body(html)
    }

    // TODO: .xml(xml: Into<Vec<u8>>) (final)
    // TODO: .text(text: Into<Vec<u8>>) (final)
}
