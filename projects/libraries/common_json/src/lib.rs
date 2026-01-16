//! # common_json
//!
//! Bibliothèque JSON complète pour Rust, avec des APIs ergonomiques,
//! une gestion d'erreurs riche et des utilitaires puissants.

// Ré-exports des modules principaux
pub mod access;
pub mod array_diff;
pub mod deserialization;
pub mod error;
pub mod json;
pub mod json_access;
pub mod json_access_mut;
pub mod json_array_builder;
pub mod json_diff;
pub mod json_object_builder;
pub mod macros;
pub mod merge;
pub mod merge_strategy;
pub mod parser;
pub mod patch_op;
pub mod process;
pub mod serialization;
pub mod value;

// Ré-exports des types et fonctions utiles
pub use value::{
    JsonArray, JsonMap, JsonNumber, JsonObject, array, boolean, null, number_f64, number_i64,
    number_u64, object, string,
};

pub use error::{JsonError, JsonResult};

pub use serialization::{
    JsonSerializable, to_bytes, to_bytes_pretty, to_json, to_json_string, to_json_string_pretty,
    to_string, to_string_pretty, to_value, value_to_bytes_pretty, write_to, write_to_pretty,
};

pub use deserialization::{
    JsonDeserializable, from_bytes, from_json, from_json_owned, from_json_str, from_reader,
    from_slice, from_str, from_value, parse, parse_bytes, parse_reader,
};

pub use merge::{concat_merge, contains, deep_merge, diff, flatten, merge, unflatten};
pub use process::parse_json_stdout;

pub use array_diff::ArrayDiff;
pub use json::Json;
pub use json_access::JsonAccess;
pub use json_access_mut::JsonAccessMut;
pub use json_diff::JsonDiff;
pub use merge_strategy::MergeStrategy;
pub use parser::parse_str;
pub use patch_op::PatchOp;
pub use value::JsonVisitor;
