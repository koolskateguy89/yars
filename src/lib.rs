mod constants;
mod error;
pub mod protocol;
// TODO?: make not public? - i need to make HTTPReq/Res public though
pub mod request;
// TODO?: make not public?
pub mod response;
mod router;
mod server;
mod status;
pub mod transport;
mod version;

pub use error::*;
pub use request::HttpRequest;
pub use response::HttpResponse;
pub use server::YarsServer;
pub use status::HttpStatusCode;
pub use version::HttpVersion;

pub type Result<T> = std::result::Result<T, Error>;
