// projects/products/unstable/autonomous_dev_ai/src/ops/run_book_entry.rs
use serde::{Deserialize, Serialize};

use crate::ops::IncidentSeverity;

/// A single runbook entry mapping a failure scenario to remediation steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunbookEntry {
    pub scenario: String,
    pub severity: IncidentSeverity,
    pub detection: String,
    pub remediation_steps: Vec<String>,
}
