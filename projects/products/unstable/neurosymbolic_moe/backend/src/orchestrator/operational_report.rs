//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/operational_report.rs
use serde::{Deserialize, Serialize};

use crate::orchestrator::ImportTelemetry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalReport {
    pub governance_current_version: u64,
    pub governance_current_checksum: Option<String>,
    pub governance_audit_entries: usize,
    pub governance_state_snapshots: usize,
    pub runtime_bundle_checksum: String,
    pub short_term_memory_entries: usize,
    pub long_term_memory_entries: usize,
    pub working_buffer_entries: usize,
    pub session_buffer_sessions: usize,
    pub session_buffer_values: usize,
    pub trace_entries: usize,
    pub dataset_entries: usize,
    pub feedback_entries: usize,
    pub import_telemetry: ImportTelemetry,
}
