// projects/products/unstable/autonomous_dev_ai/src/ops/incident_severity.rs
use serde::{Deserialize, Serialize};

/// Incident severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}
