//! `ast_core` - A generic Abstract Syntax Tree (AST) library.
//!
// projects/libraries/ast_core/src/lib.rs

mod ast_builder;
mod ast_error_kind;
mod ast_key;
mod ast_kind;
mod ast_macro;
mod ast_meta;
mod ast_node;
mod ast_path;
mod ast_span;
mod ast_validation_error;
mod conversions;
mod ext_id;
mod macros;
mod number;
mod opaque_value;
mod origin;
mod path_item;
mod validate_limits;
mod walk_validate;

pub use ast_builder::AstBuilder;
pub use ast_error_kind::AstErrorKind;
pub use ast_key::AstKey;
pub use ast_kind::AstKind;
pub use ast_meta::AstMeta;
pub use ast_node::AstNode;
pub use ast_path::AstPath;
pub use ast_span::AstSpan;
pub use ast_validation_error::AstValidationError;
pub use ext_id::ExtId;
pub use number::Number;
pub use opaque_value::OpaqueValue;
pub use origin::Origin;
pub use path_item::PathItem;
pub use validate_limits::ValidateLimits;

#[cfg(test)]
mod tests;
