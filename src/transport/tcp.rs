use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};
use tracing::{debug, info};

use super::{Transport, TransportResult};
use crate::{constants::MAX_REQUEST_SIZE, TransportError};

/// Implementation of the transport layer for TCP connections
#[derive(Default)]
pub struct TcpTransport {
    listener: Option<TcpListener>,
}

impl TcpTransport {
    pub fn new() -> Self {
        Self::default()
    }

    fn listener(&self) -> TransportResult<&TcpListener> {
        // Error should never happen because this should only be used internally
        self.listener.as_ref().ok_or(TransportError::Tcp(
            "TCP listener not bound. Call `bind` first.".into(),
        ))
    }
}

impl Transport for TcpTransport {
    type Connection = TcpStream;

    async fn bind(&mut self, local_addr: impl ToSocketAddrs) -> TransportResult<()> {
        let listener = TcpListener::bind(local_addr).await?;
        info!(
            "Listening for TCP connections on {}",
            listener.local_addr()?
        );
        self.listener = Some(listener);
        Ok(())
    }

    async fn accept(&self) -> TransportResult<Self::Connection> {
        let (stream, addr) = self.listener()?.accept().await?;
        debug!(%addr, "Accepted TCP connection");
        Ok(stream)
    }

    async fn read(&self, stream: &mut Self::Connection) -> TransportResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(MAX_REQUEST_SIZE);
        let len = stream.read_buf(&mut buf).await?;

        debug!(
            peer = %stream.peer_addr()?,
            len,
            "Successfully read from TCP connection",
        );

        Ok(buf)
    }

    async fn write(&self, stream: &mut Self::Connection, response: &[u8]) -> TransportResult<()> {
        debug!(
            peer = %stream.peer_addr()?,
            len = response.len(),
            "Writing to TCP connection",
        );
        stream.write_all(response).await.map_err(|err| err.into())
    }

    async fn shutdown_conn(&self, mut stream: Self::Connection) -> TransportResult<()> {
        stream.shutdown().await?;
        Ok(())
    }
}
