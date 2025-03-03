use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Transport(#[from] TransportError),
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Transport error: {0}")]
    Transport(String),
    #[error("TCP error: {0}")]
    Tcp(String),
}
