use log::{debug, info};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use super::{Transport, TransportResult};
use crate::TransportError;

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

    async fn bind(&mut self, addr: impl ToSocketAddrs) -> TransportResult<()> {
        let listener = TcpListener::bind(addr).await?;
        info!(
            "listening for TCP connections on {}",
            listener.local_addr()?
        );
        self.listener = Some(listener);
        Ok(())
    }

    async fn accept(&self) -> TransportResult<Self::Connection> {
        let (stream, _) = self.listener()?.accept().await?;
        debug!("accepted TCP connection from {}", stream.peer_addr()?);
        Ok(stream)
    }

    async fn read(&self, stream: &mut Self::Connection) -> TransportResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(1024);
        stream.read_buf(&mut buf).await?;

        debug!(
            "bytes read from TCP connection {}: {}",
            stream.peer_addr()?,
            buf.len(),
        );

        if buf.is_empty() {
            return Ok(vec![]);
        }

        Ok(buf)
    }

    async fn write(&self, stream: &mut Self::Connection, response: &[u8]) -> TransportResult<()> {
        debug!(
            "writing bytes to TCP connection {}: {}",
            stream.peer_addr()?,
            response.len(),
        );
        stream.write_all(response).await.map_err(|err| err.into())
    }

    async fn close(&self, _stream: Self::Connection) -> TransportResult<()> {
        todo!()
    }
}
