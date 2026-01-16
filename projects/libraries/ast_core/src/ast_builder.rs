// projects/libraries/ast_core/src/ast_builder.rs
use crate::{AstKey, AstKind, AstNode, Number};

/// Builder for creating AST nodes ergonomically.
pub struct AstBuilder;

impl AstBuilder {
    /// Creates a null node.
    pub fn null() -> AstNode {
        AstNode::new(AstKind::Null)
    }

    /// Creates a boolean node.
    pub fn bool(value: bool) -> AstNode {
        AstNode::new(AstKind::Bool(value))
    }

    /// Creates an integer node.
    pub fn int(value: i64) -> AstNode {
        AstNode::new(AstKind::Number(Number::Int(value)))
    }

    /// Creates an unsigned integer node.
    pub fn uint(value: u64) -> AstNode {
        AstNode::new(AstKind::Number(Number::Uint(value)))
    }

    /// Creates a float node.
    pub fn float(value: f64) -> AstNode {
        AstNode::new(AstKind::Number(Number::Float(value)))
    }

    /// Creates a string node.
    pub fn string(value: impl Into<String>) -> AstNode {
        AstNode::new(AstKind::String(value.into()))
    }

    /// Creates an array node.
    pub fn array(items: Vec<AstNode>) -> AstNode {
        AstNode::new(AstKind::Array(items))
    }

    /// Creates an object node.
    pub fn object<K, I>(fields: I) -> AstNode
    where
        K: Into<AstKey>,
        I: IntoIterator<Item = (K, AstNode)>,
    {
        AstNode::new(AstKind::Object(
            fields.into_iter().map(|(k, v)| (k.into(), v)).collect(),
        ))
    }

    /// Creates a node from a value (for macro use).
    pub fn from<T: Into<AstNode>>(value: T) -> AstNode {
        value.into()
    }
}
