// projects/libraries/ast_core/src/conversions/ast_node/from/str.rs
use crate::{AstBuilder, AstNode};

impl From<&str> for AstNode {
    fn from(value: &str) -> Self {
        AstBuilder::string(value)
    }
}
