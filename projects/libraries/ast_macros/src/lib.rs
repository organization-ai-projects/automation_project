//! `ast_macros` - Reusable macros for building AST nodes.
//!
//! This crate provides declarative macros for constructing and validating
//! Abstract Syntax Tree nodes. These macros are decoupled from the core AST
//! implementation and can be reused across different crates.

pub mod build_array;
pub mod build_object;
pub mod key;
pub mod validate;
pub mod value;
