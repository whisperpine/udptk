//! UDP toolkit
//!
//! This crate provides an easy-to-use API for sending and listening to UDP
//! packets. It provides a high-level interface and does not expose any low-level
//! details, such as sockets or packet headers.

// rustc
#![cfg_attr(debug_assertions, allow(unused))]
#![cfg_attr(not(debug_assertions), warn(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(clippy::unwrap_used))]
#![cfg_attr(not(debug_assertions), deny(warnings))]
// clippy
#![cfg_attr(not(debug_assertions), deny(clippy::todo))]
#![cfg_attr(
    not(any(test, debug_assertions)),
    deny(clippy::print_stdout, clippy::dbg_macro)
)]

mod error;
mod listen;
mod send;

pub use error::{Error, Result};
pub use listen::listen;
pub use send::send;
