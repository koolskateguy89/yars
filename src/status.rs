use std::collections::HashMap;

use crate::{response::HttpResponseBuilder, HttpResponse};

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

    // TODO: from num
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
