//! `ast_core` - A generic Abstract Syntax Tree (AST) library.
//!
//! This crate provides a generic AST representation and validation utilities
//! for use in procedural macros and other tools.
//!
//! # Features
//!
//! - Generic AST representation (not tied to any specific language)
//! - Validation with configurable limits (depth, size, duplicate keys)
//! - Rich metadata support (spans, origin tracking, custom attributes)
//! - Builder pattern for ergonomic AST construction
//! - Traversal and transformation utilities

mod ast_builder;
mod ast_error_kind;
mod ast_key;
mod ast_kind;
mod ast_macro;
mod ast_meta;
mod ast_node;
mod ast_path;
mod ast_validation_error;
mod ext_id;
mod number;
mod opaque_value;
mod origin;
mod path_item;
mod span;
mod validate_limits;
mod walk_validate;

pub use ast_builder::AstBuilder;
pub use ast_error_kind::AstErrorKind;
pub use ast_key::AstKey;
pub use ast_kind::AstKind;
pub use ast_meta::AstMeta;
pub use ast_node::AstNode;
pub use ast_path::AstPath;
pub use ast_validation_error::AstValidationError;
pub use ext_id::ExtId;
pub use number::Number;
pub use opaque_value::OpaqueValue;
pub use origin::Origin;
pub use path_item::PathItem;
pub use span::Span;
pub use validate_limits::ValidateLimits;

#[cfg(test)]
mod tests;
