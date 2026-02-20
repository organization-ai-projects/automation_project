// projects/libraries/ast_core/src/conversions/ast_node/from/u64.rs
use crate::{AstBuilder, AstNode};

impl From<u64> for AstNode {
    fn from(value: u64) -> Self {
        AstBuilder::uint(value)
    }
}
