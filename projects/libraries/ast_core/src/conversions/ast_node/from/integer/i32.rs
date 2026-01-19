// projects/libraries/ast_core/src/conversions/ast_node/from/i32.rs
use crate::{AstBuilder, AstNode};

impl From<i32> for AstNode {
    fn from(value: i32) -> Self {
        AstBuilder::int(value as i64)
    }
}
