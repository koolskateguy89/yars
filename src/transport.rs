//! Transport layer
//!
//! Supported protocols:
//! - TCP

mod tcp;

use crate::Result;

/// Generic transport layer
pub(crate) trait Transport {
    /// TODO
    type Connection;

    /// TODO
    async fn accept(&self) -> Result<Self::Connection>;

    /// TODO
    async fn read(&self, conn: &mut Self::Connection) -> Result<Vec<u8>>;

    /// TODO
    async fn write(&self, conn: &mut Self::Connection, response: &[u8]) -> Result<()>;

    /// TODO
    async fn close(&self, conn: Self::Connection) -> Result<()>;
}
