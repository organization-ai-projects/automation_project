// projects/products/unstable/protocol_builder/backend/src/schema/mod.rs
#![allow(clippy::module_inception)]

pub mod endpoint_spec;
pub mod field_spec;
pub mod message_spec;
pub mod schema;
pub mod type_spec;

pub use schema::Schema;
