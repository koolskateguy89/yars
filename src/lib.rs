//! # YARS - Yet Another Rust Server
//!
//! An asynchronous server framework for Rust.
//!
//! Tracing is done using the [`tracing`] crate.
//! Make sure to set up a subscriber before running the server.

mod constants;
mod error;
mod router;
mod server;

pub mod http;
pub mod prelude;
pub mod protocol;
pub mod transport;

pub use prelude::*;
