use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryType {
    Short,
    Medium,
    Long,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub memory_type: MemoryType,
    pub relevance: f64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub tags: Option<Vec<String>>,
    pub memory_type: Option<MemoryType>,
    pub min_relevance: Option<f64>,
    pub max_results: usize,
    pub include_expired: bool,
    pub current_time: Option<u64>,
}
