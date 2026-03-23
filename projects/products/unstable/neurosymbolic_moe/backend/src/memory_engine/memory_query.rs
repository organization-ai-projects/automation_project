use serde::{Deserialize, Serialize};

use super::memory_type::MemoryType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub tags: Option<Vec<String>>,
    pub memory_type: Option<MemoryType>,
    pub min_relevance: Option<f64>,
    pub max_results: usize,
    pub include_expired: bool,
    pub current_time: Option<u64>,
}
