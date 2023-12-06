use std::collections::HashMap;

#[derive(Debug)]
pub struct HTTPRequest {
    // TODO
    pub method: RequestMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy, Hash)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
}

// TODO: pub(crate) stuff for creating request
