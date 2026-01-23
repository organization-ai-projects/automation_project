// projects/libraries/ast_core/src/ast_kind.rs
use crate::{AstKey, AstNode, Number, OpaqueValue};

/// The kind/type of an AST node.
#[derive(Clone, Debug, PartialEq)]
pub enum AstKind {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<AstNode>),
    Object(Vec<(AstKey, AstNode)>),
    Opaque(OpaqueValue),
}
