use crate::{AstBuilder, AstNode};

impl From<u8> for AstNode {
    fn from(value: u8) -> Self {
        AstBuilder::uint(value as u64)
    }
}
