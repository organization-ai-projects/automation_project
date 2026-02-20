/// projects/libraries/ast_core/src/path_item.rs
/// An item in an AST path.
#[derive(Clone, Debug, PartialEq)]
pub enum PathItem {
    Key(String),
    Index(usize),
}
