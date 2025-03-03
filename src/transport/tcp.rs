use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use super::Transport;
use crate::Result;

/// Implementation of the transport layer for TCP connections
pub(crate) struct TcpTransport {
    listener: TcpListener,
}

impl TcpTransport {
    pub async fn new(addr: impl ToSocketAddrs) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }
}

impl Transport for TcpTransport {
    type Connection = TcpStream;

    async fn accept(&self) -> Result<Self::Connection> {
        let (stream, _) = self.listener.accept().await?;
        Ok(stream)
    }

    async fn read(&self, conn: &mut Self::Connection) -> Result<Vec<u8>> {
        todo!()
    }

    async fn write(&self, conn: &mut Self::Connection, response: &[u8]) -> Result<()> {
        todo!()
    }

    async fn close(&self, conn: Self::Connection) -> Result<()> {
        todo!()
    }
}

fn hello() {}
