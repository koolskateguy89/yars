//! Transport layer
//!
//! Supported protocols:
//! - TCP

mod tcp;

use tokio::net::ToSocketAddrs;

use crate::Result;

pub use tcp::TcpTransport;

/// Generic transport layer
///
/// Only transport that use socket addresses (e.g. TCP, UDP) can be supported, because of
/// [Transport::bind] accepting an argument that implements [ToSocketAddrs].
pub(crate) trait Transport {
    /// TODO
    type Connection;

    /// Bind the transport to its listening address.
    async fn bind(&mut self, addr: impl ToSocketAddrs) -> Result<()>;

    /// TODO
    async fn accept(&self) -> Result<Self::Connection>;

    /// TODO
    async fn read(&self, conn: &mut Self::Connection) -> Result<Vec<u8>>;

    /// TODO
    async fn write(&self, conn: &mut Self::Connection, response: &[u8]) -> Result<()>;

    /// TODO
    async fn close(&self, conn: Self::Connection) -> Result<()>;
}
