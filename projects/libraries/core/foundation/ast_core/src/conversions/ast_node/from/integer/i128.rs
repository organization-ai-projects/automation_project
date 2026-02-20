// projects/libraries/ast_core/src/conversions/ast_node/from/i128.rs
use crate::{AstBuilder, AstNode};

impl From<i128> for AstNode {
    fn from(value: i128) -> Self {
        if value >= i64::MIN as i128 && value <= i64::MAX as i128 {
            AstBuilder::int(value as i64)
        } else {
            AstBuilder::string(value.to_string())
        }
    }
}
