mod constants;
mod error;
mod router;
mod server;
mod status;

pub mod protocol;
// TODO?: make not public? - i need to make HTTPReq/Res public though
pub mod request;
// TODO?: make not public?
pub mod response;
pub mod transport;

pub use error::*;
pub use request::HttpRequest;
pub use response::HttpResponse;
pub use server::YarsServer;
// TODO: move http
pub use status::HttpStatusCode;

pub type Result<T> = std::result::Result<T, Error>;
