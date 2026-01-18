use crate::{AstBuilder, AstNode};

impl From<i64> for AstNode {
    fn from(value: i64) -> Self {
        AstBuilder::int(value)
    }
}
