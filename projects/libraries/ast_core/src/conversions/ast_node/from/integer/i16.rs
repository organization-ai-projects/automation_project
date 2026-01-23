// projects/libraries/ast_core/src/conversions/ast_node/from/i16.rs
use crate::{AstBuilder, AstNode};

impl From<i16> for AstNode {
    fn from(value: i16) -> Self {
        AstBuilder::int(value as i64)
    }
}
