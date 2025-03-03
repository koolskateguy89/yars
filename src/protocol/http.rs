// TODO: move these to be under some /http
use crate::{HttpRequest, HttpResponse};

use super::Protocol;

#[derive(Default)]
pub struct HttpProtocol;

impl HttpProtocol {
    pub fn new() -> Self {
        todo!()
    }
}

impl Protocol for HttpProtocol {
    type Req = HttpRequest;

    type Res = HttpResponse;

    fn parse_request(&self, raw: &[u8]) -> Option<Self::Req> {
        todo!()
    }

    fn serialize_response(&self, response: &Self::Res) -> Vec<u8> {
        todo!()
    }
}
