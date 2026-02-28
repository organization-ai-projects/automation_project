// projects/products/unstable/protocol_builder/backend/src/schema/type_spec.rs
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeSpec {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Bytes,
}

impl fmt::Display for TypeSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TypeSpec::U8 => "u8",
            TypeSpec::U16 => "u16",
            TypeSpec::U32 => "u32",
            TypeSpec::U64 => "u64",
            TypeSpec::I8 => "i8",
            TypeSpec::I16 => "i16",
            TypeSpec::I32 => "i32",
            TypeSpec::I64 => "i64",
            TypeSpec::F32 => "f32",
            TypeSpec::F64 => "f64",
            TypeSpec::Bool => "bool",
            TypeSpec::String => "String",
            TypeSpec::Bytes => "Vec<u8>",
        };
        write!(f, "{}", s)
    }
}
