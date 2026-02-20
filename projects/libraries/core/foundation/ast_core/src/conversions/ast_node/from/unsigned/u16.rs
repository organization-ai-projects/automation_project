// projects/libraries/ast_core/src/conversions/ast_node/from/u16.rs
use crate::{AstBuilder, AstNode};

impl From<u16> for AstNode {
    fn from(value: u16) -> Self {
        AstBuilder::uint(value as u64)
    }
}
