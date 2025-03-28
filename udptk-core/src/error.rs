/// A handy type alias for `Result<T, udptk::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Enumeration of errors that can occur in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error happened during io operation.
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    /// Error happened during utf-8 conversion.
    #[error("utf-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// Cannot find ip address for given domain.
    #[error("cannot find ip address for domain: {0}")]
    NoIpAddress(String),
    /// Cannot find free udp socket to bind.
    #[error("cannot find free udp socket to bind")]
    NoFreeSocket,
    /// Unknown error happened.
    #[error("unknown error")]
    Unknown,
}
