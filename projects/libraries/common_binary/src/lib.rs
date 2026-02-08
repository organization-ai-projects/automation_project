//! # common_binary
//!
//! Stable high-performance binary persistence ABI for the workspace.
//!
//! Provides a single canonical binary persistence API that is:
//! - **Performance-first**: Fast load/store, minimal allocations
//! - **Safe**: Detects corruption and incompatibility early
//! - **Backend-agnostic**: Implementation hidden behind the ABI
//!
//! ## Example
//!
//! ```rust
//! use common_binary::{BinaryOptions, BinaryEncode, BinaryDecode, write_binary, read_binary};
//! use std::path::Path;
//!
//! #[derive(Debug, PartialEq, bincode::Encode, bincode::Decode)]
//! struct MyData {
//!     value: u64,
//!     name: String,
//! }
//!
//! impl BinaryEncode for MyData {
//!     fn encode_binary(&self, out: &mut Vec<u8>) -> Result<(), common_binary::BinaryError> {
//!         bincode::encode_into_std_write(self, out, bincode::config::standard())
//!             .map(|_| ())
//!             .map_err(|e| common_binary::BinaryError::Encode(Box::leak(e.to_string().into_boxed_str())))
//!     }
//! }
//!
//! impl BinaryDecode for MyData {
//!     fn decode_binary(input: &[u8]) -> Result<Self, common_binary::BinaryError> {
//!         bincode::decode_from_slice(input, bincode::config::standard())
//!             .map(|(data, _)| data)
//!             .map_err(|e| common_binary::BinaryError::Decode(Box::leak(e.to_string().into_boxed_str())))
//!     }
//! }
//! ```

mod error;
mod header;
mod io;
mod options;
mod traits;

pub use error::BinaryError;
pub use options::BinaryOptions;
pub use traits::{BinaryDecode, BinaryEncode};
pub use io::{read_binary, write_binary};

#[cfg(test)]
mod tests;
