//! Transport layer
//!
//! Allows user-defined protocols in transport layer.
//!
//! Supported protocols:
//! - TCP

use std::future::Future;

use tokio::net::ToSocketAddrs;

mod tcp;

use crate::TransportError;

pub use tcp::TcpTransport;

pub type TransportResult<T> = std::result::Result<T, TransportError>;

/// Generic transport layer
///
/// Only transport that use socket addresses (e.g. TCP, UDP) can be supported, because of
/// [Transport::bind] accepting an argument that implements [ToSocketAddrs].
pub trait Transport: Send + Sync + 'static {
    /// TODO
    type Connection: Send + Sync;

    /// Bind the transport to its listening address.
    ///
    /// Should provide a detailed log message saying that the transport is listening on the given address.
    async fn bind(&mut self, addr: impl ToSocketAddrs) -> TransportResult<()>;

    /// TODO
    async fn accept(&self) -> TransportResult<Self::Connection>;

    /// TODO
    fn read(
        &self,
        conn: &mut Self::Connection,
    ) -> impl Future<Output = TransportResult<Vec<u8>>> + Send;

    /// TODO
    fn write(
        &self,
        conn: &mut Self::Connection,
        response: &[u8],
    ) -> impl Future<Output = TransportResult<()>> + Send;

    /// TODO
    async fn close(&self, conn: Self::Connection) -> TransportResult<()>;
}
