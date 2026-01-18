// projects/libraries/ast_core/src/conversions/ast_node/from/isize.rs
use crate::{AstBuilder, AstNode};

impl From<isize> for AstNode {
    fn from(value: isize) -> Self {
        if value >= 0 {
            AstBuilder::uint(value as u64)
        } else {
            AstBuilder::int(value as i64)
        }
    }
}
