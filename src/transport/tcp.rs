use log::{debug, info};


use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use super::Transport;
use crate::{Result, TransportError};

/// Implementation of the transport layer for TCP connections
#[derive(Default)]
pub struct TcpTransport {
    listener: Option<TcpListener>,
}

impl TcpTransport {
    fn listener(&self) -> Result<&TcpListener> {
        self.listener
            .as_ref()
            .ok_or(TransportError::Tcp("TCP listener not bound. Call `bind` first.".into()).into())
    }
}

impl Transport for TcpTransport {
    type Connection = TcpStream;

    async fn bind(&mut self, addr: impl ToSocketAddrs) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        self.listener = Some(listener);
        Ok(())
    }

    async fn accept(&self) -> Result<Self::Connection> {
        let (stream, _) = self.listener()?.accept().await?;
        Ok(stream)
    }

    async fn read(&self, stream: &mut Self::Connection) -> Result<Vec<u8>> {
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

    async fn write(&self, stream: &mut Self::Connection, response: &[u8]) -> Result<()> {
        stream.write_all(response).await.map_err(|err| err.into())
    }

    async fn close(&self, _stream: Self::Connection) -> Result<()> {
        todo!()
    }
}
