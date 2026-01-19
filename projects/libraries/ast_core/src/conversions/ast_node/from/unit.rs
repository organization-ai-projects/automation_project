// projects/libraries/ast_core/src/conversions/ast_node/from/unit.rs
use crate::{AstBuilder, AstNode};

impl From<()> for AstNode {
    fn from(_: ()) -> Self {
        AstBuilder::null()
    }
}
