//! Transport layer
//!
//! Allows user-defined protocols in transport layer.
//!
//! Supported protocols:
//! - TCP

use std::net::SocketAddr;

use tokio::net::ToSocketAddrs;

use crate::TransportError;

mod tcp;

pub use tcp::TcpTransport;

pub type TransportResult<T> = std::result::Result<T, TransportError>;

/// Generic transport layer
///
/// Only transport that use socket addresses (e.g. TCP, UDP) can be supported, because of
/// [Transport::bind] accepting an argument that implements [ToSocketAddrs].
pub trait Transport {
    /// TODO
    type Connection;

    /// Bind the transport to its listening address.
    async fn bind(&mut self, addr: impl ToSocketAddrs) -> TransportResult<()>;

    /// TODO
    async fn accept(&self) -> TransportResult<Self::Connection>;

    /// TODO
    async fn read(&self, conn: &mut Self::Connection) -> TransportResult<Vec<u8>>;

    /// TODO
    async fn write(&self, conn: &mut Self::Connection, response: &[u8]) -> TransportResult<()>;

    /// TODO
    async fn close(&self, conn: Self::Connection) -> TransportResult<()>;
}
