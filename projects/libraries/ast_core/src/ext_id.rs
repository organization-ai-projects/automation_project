// projects/libraries/ast_core/src/ext_id.rs
// An extension identifier for opaque values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExtId(pub u128);
