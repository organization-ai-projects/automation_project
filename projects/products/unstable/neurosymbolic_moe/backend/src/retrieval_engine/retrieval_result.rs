use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalResult {
    pub chunk_id: String,
    pub content: String,
    pub relevance_score: f64,
    pub source: String,
    pub metadata: HashMap<String, String>,
}

impl RetrievalResult {
    pub fn new(
        chunk_id: impl Into<String>,
        content: impl Into<String>,
        relevance_score: f64,
        source: impl Into<String>,
    ) -> Self {
        Self {
            chunk_id: chunk_id.into(),
            content: content.into(),
            relevance_score,
            source: source.into(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
