// projects/libraries/ast_core/src/conversions/ast_node/from/i8.rs
use crate::{AstBuilder, AstNode};

impl From<i8> for AstNode {
    fn from(value: i8) -> Self {
        AstBuilder::int(value as i64)
    }
}
