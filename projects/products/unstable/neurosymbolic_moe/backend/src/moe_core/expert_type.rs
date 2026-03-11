use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpertType {
    Deterministic,
    Symbolic,
    Neural,
    Hybrid,
}
