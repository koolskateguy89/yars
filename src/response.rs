use std::collections::HashMap;

// TODO: type builder thingy so status is required
#[derive(Clone, Debug, Default)]
pub struct HTTPResponse {
    status: Option<u32>, // idk if u32 is right
    headers: HashMap<String, String>,
    // TODO?: change to bytes?
    body: Option<String>,
}

impl HTTPResponse {
    pub fn new() -> Self {
        Self {
            status: None,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn headers<T>(mut self, headers: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.headers = headers.into_iter().collect();
        self
    }

    pub fn status(mut self, status: u32) -> Self {
        self.status = Some(status);
        self
    }

    // TODO: .json(json: String) (final)
    // TODO: .xml(xml: String) (final)
    // TODO: .html(html: String) (final)
    // TODO: .text(text: String) (final)
    // TODO: .body(body: String) (final - doesnt set content type)

    // TODO?: some way to send binary data
}

// TODO: tests
