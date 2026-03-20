//! projects/products/unstable/neurosymbolic_moe/backend/src/retrieval_engine/retrieval_query.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId};

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
