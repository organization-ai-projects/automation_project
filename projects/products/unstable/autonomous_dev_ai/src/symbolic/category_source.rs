use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategorySource {
    DeterministicLabels,
    LatentHeuristic,
    Unknown,
}
