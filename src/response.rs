use std::collections::HashMap;

// TODO!: Status code enum

// TODO: type builder thingy so status is required
#[derive(Clone, Debug, Default)]
pub struct HttpResponse {
    status: Option<u32>, // idk if u32 is right
    headers: HashMap<String, String>,
    // TODO?: change to bytes?
    body: Option<String>,
}

impl HttpResponse {
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

// Response:
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body

// pub trait ToResponse {
//     fn to_response(self) -> HttpResponse;
// }

// impl<T> ToResponse for T
// where
//     T: Into<String>,
// {
//     fn to_response(self) -> HttpResponse {
//         HttpResponse {
//             status: Some(200),
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
            status: Some(200),
            headers: HashMap::new(),
            body: Some(body.into()),
        }
    }
}

// TODO: tests
