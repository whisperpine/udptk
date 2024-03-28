use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum UdptkError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("unknown error")]
    Unknown,
}
