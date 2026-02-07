//! `ast_macros` - Reusable macros for building AST nodes.
//!
//! This crate provides declarative macros for constructing and validating
//! Abstract Syntax Tree nodes. These macros are decoupled from the core AST
//! implementation and can be reused across different crates.

mod build_array;
mod build_object;
mod key;
mod validate;
mod value;
