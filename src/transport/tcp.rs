use log::debug;

use bytes::BytesMut;
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
    fn listener(&self) -> TransportResult<&TcpListener> {
        self.listener.as_ref().ok_or(TransportError::Tcp(
            "TCP listener not bound. Call `bind` first.".into(),
        ))
    }
}

impl Transport for TcpTransport {
    type Connection = TcpStream;

    async fn bind(&mut self, addr: impl ToSocketAddrs) -> TransportResult<()> {
        let listener = TcpListener::bind(addr).await?;
        self.listener = Some(listener);
        Ok(())
    }

    async fn accept(&self) -> TransportResult<Self::Connection> {
        let (stream, _) = self.listener()?.accept().await?;
        Ok(stream)
    }

    async fn read(&self, stream: &mut Self::Connection) -> TransportResult<Vec<u8>> {
        let mut buf = BytesMut::with_capacity(1024);
        stream.read_buf(&mut buf).await?;

        if buf.is_empty() {
            return Ok(vec![]);
        }

        debug!(
            "bytes read from connection with {}: {}",
            stream.peer_addr()?,
            buf.len()
        );

        Ok(buf.to_vec())
    }

    async fn write(&self, stream: &mut Self::Connection, response: &[u8]) -> TransportResult<()> {
        stream.write_all(response).await.map_err(|err| err.into())
    }

    async fn close(&self, _stream: Self::Connection) -> TransportResult<()> {
        todo!()
    }
}
