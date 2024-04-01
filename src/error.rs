use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum UdptkError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("cannot find ip address for domain: {0}")]
    NoIpAddress(String),
    #[error("cannot find free udp socket to bind")]
    NoFreeSocket,
    #[error("unknown error")]
    Unknown,
}
