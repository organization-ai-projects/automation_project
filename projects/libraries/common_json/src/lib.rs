//! # common_json
//!
//! Bibliothèque JSON complète pour Rust, avec des APIs ergonomiques,
//! une gestion d'erreurs riche et des utilitaires puissants.

// Ré-exports des modules principaux
pub mod access;
pub mod array_diff;
pub mod deserialize;
pub mod error;
pub mod json_access;
pub mod json_access_mut;
pub mod json_array_builder;
pub mod json_diff;
pub mod json_object_builder;
pub mod macros;
pub mod merge;
pub mod merge_strategy;
pub mod patch_op;
mod parser;
pub mod serialize;
pub mod value;

// Ré-exports des types et fonctions utiles
pub use value::{
    Json, JsonArray, JsonMap, JsonNumber, JsonObject, array, boolean, null, number_f64, number_i64,
    number_u64, object, string,
};

pub use error::{JsonError, JsonResult};

pub use serialize::{
    JsonSerializable, to_bytes, to_bytes_pretty, to_json, to_json_string, to_json_string_pretty,
    to_string, to_string_pretty, to_value, value_to_bytes_pretty, write_to, write_to_pretty,
};

pub use deserialize::{
    JsonDeserializable, from_bytes, from_json, from_json_owned, from_json_str, from_reader,
    from_str, from_value, parse, parse_bytes, parse_reader,
};

pub use merge::{concat_merge, contains, deep_merge, diff, flatten, merge, unflatten};

// Prelude pour les imports globaux
pub mod prelude {
    //! Ré-exports pratiques pour import glob.
    pub use crate::deserialize::JsonDeserializable;
    pub use crate::error::{JsonError, JsonResult};
    pub use crate::pjson;
    pub use crate::serialize::JsonSerializable;
    pub use crate::value::{Json, JsonArray, JsonMap, JsonObject};
}
