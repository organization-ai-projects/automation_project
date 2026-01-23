// projects/libraries/ast_core/src/conversions/ast_node/from/f32.rs
use crate::{AstBuilder, AstNode};

impl From<f32> for AstNode {
    fn from(value: f32) -> Self {
        AstBuilder::float(value as f64)
    }
}
