use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Transport(#[from] TransportError),

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
