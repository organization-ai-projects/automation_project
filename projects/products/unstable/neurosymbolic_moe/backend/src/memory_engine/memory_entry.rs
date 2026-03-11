use super::memory_type::MemoryType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
