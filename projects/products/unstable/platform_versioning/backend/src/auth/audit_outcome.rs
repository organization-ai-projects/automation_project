// projects/products/unstable/platform_versioning/backend/src/auth/audit_outcome.rs
use serde::{Deserialize, Serialize};

/// Whether an audited action succeeded or was denied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    Allowed,
    Denied,
}
