use crate::relations::relationship_kind::RelationshipKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub kind: RelationshipKind,
    /// Score in range -100..=100
    pub score: i32,
}

impl Relationship {
    #[allow(dead_code)]
    pub fn new(kind: RelationshipKind, score: i32) -> Self {
        Self {
            kind,
            score: score.clamp(-100, 100),
        }
    }
}
