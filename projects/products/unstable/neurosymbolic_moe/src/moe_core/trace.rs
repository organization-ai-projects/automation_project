use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::expert::ExpertId;
use super::task::TaskId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TracePhase {
    Routing,
    Retrieval,
    MemoryQuery,
    ExpertSelection,
    ExpertExecution,
    Aggregation,
    Validation,
    Feedback,
    DatasetEnrichment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRecord {
    pub trace_id: String,
    pub task_id: TaskId,
    pub timestamp: u64,
    pub expert_id: Option<ExpertId>,
    pub phase: TracePhase,
    pub detail: String,
    pub metadata: HashMap<String, String>,
}
