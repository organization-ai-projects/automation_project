// projects/libraries/ast_core/src/conversions/ast_node/from/f64.rs
use crate::{AstBuilder, AstNode};

impl From<f64> for AstNode {
    fn from(value: f64) -> Self {
        AstBuilder::float(value)
    }
}
