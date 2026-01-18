// projects/libraries/ast_core/src/conversions/ast_node/from/u16.rs
use crate::{AstBuilder, AstNode};

impl From<u32> for AstNode {
    fn from(value: u32) -> Self {
        AstBuilder::uint(value as u64)
    }
}
