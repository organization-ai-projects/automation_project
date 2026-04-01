/// A positioned node with integer coordinates for determinism.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePosition {
    pub id: String,
    pub x: i64,
    pub y: i64,
}
