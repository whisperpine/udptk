//! UDP toolkit
//!
//! This crate provides an easy-to-use API for sending and listening to UDP
//! packets. It provides a high-level interface and does not expose any low-level
//! details, such as sockets or packet headers.

#![cfg_attr(debug_assertions, allow(unused))]
#![cfg_attr(not(debug_assertions), deny(clippy::unwrap_used))]

pub mod error;
pub mod listen;
pub mod send;

pub use error::UdptkError;
pub use listen::listen;
pub use send::send;

/// The version of this crate as a string.
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
