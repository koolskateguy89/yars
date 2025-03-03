use super::Protocol;

pub struct HttpProtocol;

impl HttpProtocol {
    pub fn new() -> Self {
        todo!()
    }
}

impl Protocol for HttpProtocol {
    type Req = HttpProtocol;

    type Res = HttpProtocol;

    fn parse_request(&self, raw: &[u8]) -> Option<Self::Req> {
        todo!()
    }

    fn serialize_response(&self, response: &Self::Res) -> Vec<u8> {
        todo!()
    }
}
