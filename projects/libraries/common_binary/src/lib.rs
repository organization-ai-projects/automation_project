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
//! use serde::{Serialize, Deserialize};
//! use std::path::Path;
//!
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct MyData {
//!     value: u64,
//!     name: String,
//! }
//!
//! // BinaryEncode and BinaryDecode are automatically implemented
//! // for any type that implements Serialize and Deserialize
//! ```

mod deserializer;
mod error;
mod header;
mod io;
mod options;
mod serializer;
mod traits;

pub use error::BinaryError;
pub use io::{read_binary, write_binary};
pub use options::BinaryOptions;
pub use traits::{BinaryDecode, BinaryEncode};

#[cfg(test)]
mod tests;
