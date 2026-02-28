use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipKind {
    Friend,
    Enemy,
    Acquaintance,
    Romantic,
    Family,
}
