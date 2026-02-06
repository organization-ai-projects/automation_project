// projects/products/unstable/auto_manager_ai/src/domain/evidence.rs

use serde::{Deserialize, Serialize};

/// Evidence source for an action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    pub source: String,
    pub pointer: String,
}
