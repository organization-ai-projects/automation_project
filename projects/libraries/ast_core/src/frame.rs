// projects/libraries/ast_core/src/frame.rs
use crate::AstNode;

pub(crate) enum Frame<'a> {
    Enter {
        node: &'a AstNode,
        depth: usize,
    },
    ArrayNext {
        node: &'a AstNode,
        depth: usize,
        idx: usize,
    },
    ObjectNext {
        node: &'a AstNode,
        depth: usize,
        idx: usize,
    },
    PopPath,
}
