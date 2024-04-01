#![cfg_attr(debug_assertions, allow(unused))]
#![cfg_attr(not(debug_assertions), deny(clippy::unwrap_used))]

pub mod cli;
pub mod error;
pub mod listen;
pub mod send;

pub use error::UdptkError;
pub use listen::listen;
pub use send::send;

/// Program version.
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");
