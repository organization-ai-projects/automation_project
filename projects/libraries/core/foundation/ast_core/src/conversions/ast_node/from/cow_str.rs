// projects/libraries/ast_core/src/conversions/ast_node/from/cow_str.rs
use crate::{AstBuilder, AstNode};
use std::borrow::Cow;

impl<'a> From<Cow<'a, str>> for AstNode {
    fn from(value: Cow<'a, str>) -> Self {
        AstBuilder::string(value.into_owned())
    }
}
