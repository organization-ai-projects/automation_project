// projects/products/unstable/autonomous_dev_ai/src/symbolic/category_source.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategorySource {
    DeterministicLabels,
    LatentHeuristic,
    Unknown,
}
