use serde::{Deserialize, Serialize};

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
