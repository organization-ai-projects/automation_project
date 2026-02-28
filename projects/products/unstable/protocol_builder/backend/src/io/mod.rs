// projects/products/unstable/protocol_builder/backend/src/io/mod.rs
pub mod fs_writer;
pub mod json_codec;

pub use fs_writer::write_atomic;
pub use json_codec::{decode, encode};
