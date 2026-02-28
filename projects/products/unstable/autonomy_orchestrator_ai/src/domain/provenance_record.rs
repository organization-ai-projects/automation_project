// projects/products/unstable/autonomy_orchestrator_ai/src/domain/provenance_record.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub id: String,
    pub event_type: String,
    pub parent_ids: Vec<String>,
    pub reason_codes: Vec<String>,
    pub artifact_refs: Vec<String>,
    pub timestamp_unix_secs: u64,
}
