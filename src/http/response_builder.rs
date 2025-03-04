use std::collections::HashMap;

use super::{HttpResponse, HttpStatusCode};

pub struct HttpResponseBuilder {
    pub(crate) status: HttpStatusCode,
    // TODO?: Vec<u8> instead of String?
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

    /// Adds header `Content-Type: application/json`.
    pub fn json(self, json: impl Into<Vec<u8>>) -> HttpResponse {
        self.header("Content-Type", "application/json").body(json)
    }

    /// Adds header `Content-Type: text/html`.
    pub fn html(self, html: impl Into<Vec<u8>>) -> HttpResponse {
        self.header("Content-Type", "text/html").body(html)
    }

    /// Adds header `Content-Type: application/xml`.
    ///
    /// If you want `text/xml`, add a custom header instead with [`Self::header()`]
    pub fn xml(self, xml: impl Into<Vec<u8>>) -> HttpResponse {
        self.header("Content-Type", "application/xml").body(xml)
    }

    /// Adds header `Content-Type: text/plain`.
    pub fn text(self, text: impl Into<Vec<u8>>) -> HttpResponse {
        self.header("Content-Type", "text/plain").body(text)
    }
}

// TODO: tests
