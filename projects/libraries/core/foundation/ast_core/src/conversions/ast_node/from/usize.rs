// projects/libraries/ast_core/src/conversions/ast_node/from/usize.rs
use crate::{AstBuilder, AstNode};

impl From<usize> for AstNode {
    fn from(value: usize) -> Self {
        if value <= u64::MAX as usize {
            AstBuilder::uint(value as u64)
        } else {
            AstBuilder::string(value.to_string())
        }
    }
}
