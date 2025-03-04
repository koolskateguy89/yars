use std::collections::HashMap;

use super::{response_builder::HttpResponseBuilder, HttpResponse};

macro_rules! status_codes {
    (
        $(
            ($num:expr, $name:ident, $phrase:expr);
        )+
    ) => {
/// HTTP status codes
///
/// <https://tools.ietf.org/html/rfc2616#section-10>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatusCode {
    $(
        $name,
    )+
}

impl HttpStatusCode {
    pub fn phrase(&self) -> &'static str {
        match self {
            $(
                Self::$name => $phrase,
            )+
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            $(
                Self::$name => $num,
            )+
        }
    }
}

impl TryFrom<u16> for HttpStatusCode {
    type Error = &'static str;

    fn try_from(code: u16) -> Result<Self, Self::Error> {
        match code {
            $(
                $num => Ok(Self::$name),
            )+
            _ => Err("Invalid or not implemented status code"),
        }
    }
}

impl HttpResponse {
$(
    #[allow(non_snake_case)]
    pub fn $name() -> HttpResponseBuilder {
        HttpResponseBuilder {
            status: HttpStatusCode::$name,
            headers: HashMap::new(),
        }
    }
)+
}

    }
}

status_codes! {
    // TODO: all
    (100, Continue, "Continue");
    (200, Ok, "OK");
    (404, NotFound, "Not Found");
    (500, InternalServerError, "Internal Server Error");
}

#[allow(clippy::derivable_impls)]
impl Default for HttpStatusCode {
    fn default() -> Self {
        HttpStatusCode::Ok
    }
}
