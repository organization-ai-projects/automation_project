use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExpertType {
    Deterministic,
    Symbolic,
    Neural,
    Hybrid,
}
