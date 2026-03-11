use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::moe_core::{ExpertId, MoeError, TaskId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalQuery {
    pub query: String,
    pub task_id: Option<TaskId>,
    pub expert_id: Option<ExpertId>,
    pub max_results: usize,
    pub min_relevance: f64,
    pub filters: HashMap<String, String>,
}

impl RetrievalQuery {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            task_id: None,
            expert_id: None,
            max_results: 10,
            min_relevance: 0.0,
            filters: HashMap::new(),
        }
    }

    pub fn with_task_id(mut self, task_id: TaskId) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn with_expert_id(mut self, expert_id: ExpertId) -> Self {
        self.expert_id = Some(expert_id);
        self
    }

    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.max_results = max_results;
        self
    }

    pub fn with_min_relevance(mut self, min_relevance: f64) -> Self {
        self.min_relevance = min_relevance;
        self
    }

    pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.filters.insert(key.into(), value.into());
        self
    }
}

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

pub trait Retriever {
    fn retrieve(&self, query: &RetrievalQuery) -> Result<Vec<RetrievalResult>, MoeError>;
}
