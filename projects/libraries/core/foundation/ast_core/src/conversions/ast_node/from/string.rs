// projects/libraries/ast_core/src/conversions/ast_node/from/string.rs
use crate::{AstBuilder, AstNode};

impl From<String> for AstNode {
    fn from(value: String) -> Self {
        AstBuilder::string(value)
    }
}
