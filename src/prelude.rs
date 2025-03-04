use crate::{error, server};

pub use error::*;
pub use server::YarsServer;

pub type Result<T> = std::result::Result<T, Error>;
