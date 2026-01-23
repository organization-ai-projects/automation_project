// projects/libraries/ast_core/src/conversions/ast_node/from/bool.rs
use crate::{AstBuilder, AstNode};

impl From<bool> for AstNode {
    fn from(value: bool) -> Self {
        AstBuilder::bool(value)
    }
}
