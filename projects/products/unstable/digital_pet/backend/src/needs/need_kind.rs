// projects/products/unstable/digital_pet/backend/src/needs/need_kind.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeedKind {
    Hunger,
    Fatigue,
    Happiness,
    Discipline,
}
