use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::expert_id::ExpertId;
use super::task_id::TaskId;
use super::trace_phase::TracePhase;

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
