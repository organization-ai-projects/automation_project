// projects/libraries/ast_core/src/conversions/ast_node/from/u128.rs
use crate::{AstBuilder, AstNode};

impl From<u128> for AstNode {
    fn from(value: u128) -> Self {
        if value <= u64::MAX as u128 {
            AstBuilder::uint(value as u64)
        } else {
            AstBuilder::string(value.to_string())
        }
    }
}
