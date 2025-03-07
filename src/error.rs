use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error(transparent)]
    Protocol(#[from] ProtocolError),

    #[error("Handler error: {0}")]
    Handler(String),
    // TODO: rest
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Transport error: {0}")]
    Generic(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("TCP error: {0}")]
    Tcp(String),
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Protocol error: {0}")]
    Generic(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("TCP error: {0}")]
    Tcp(String),
}
