//! # common_json
//!
//! Comprehensive JSON library for Rust, with ergonomic APIs,
//! rich error handling, and powerful utilities.
// projects/libraries/common_json/src/lib.rs

pub mod access;
pub mod deserialization;
pub mod json;
pub mod json_access;
pub mod json_access_mut;
pub mod json_array_builder;
pub mod json_comparison;
pub mod json_error;
pub mod json_error_code;
mod json_number;
mod json_number_visitor;
pub mod json_object_builder;
mod json_visitor;
pub mod macros;
pub mod merge;
pub mod merge_strategy;
pub mod parser;
pub mod patch_op;
pub mod process;
pub mod serialization;
pub mod value;

pub use value::{
    JsonArray, JsonMap, JsonNumber, JsonObject, array, boolean, null, number_f64, number_i64,
    number_u64, object, string,
};

pub use json_error::{JsonError, JsonResult};

pub use serialization::{
    JsonSerializable, to_bytes, to_bytes_pretty, to_json, to_json_string, to_json_string_pretty,
    to_string, to_string_pretty, to_value, value_to_bytes_pretty, write_to, write_to_pretty,
};

pub use deserialization::{
    JsonDeserializable, from_bytes, from_json, from_json_owned, from_json_str, from_reader,
    from_slice, from_str, from_value, parse, parse_bytes, parse_reader,
};

pub use merge::{contains, flatten, merge, unflatten};

pub use process::parse_json_stdout;

pub use json::Json;
pub use json_access::JsonAccess;
pub use json_access_mut::JsonAccessMut;
pub use json_comparison::JsonComparison;
pub use merge_strategy::MergeStrategy;
pub use parser::parse_str;
pub use patch_op::PatchOp;
pub use value::JsonVisitor;

#[cfg(test)]
mod tests;
