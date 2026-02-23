// projects/products/unstable/platform_versioning/backend/src/auth/audit_entry.rs
use serde::{Deserialize, Serialize};

use crate::auth::AuditOutcome;
use crate::ids::RepoId;

/// A single audit log entry recording a sensitive action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unix timestamp of the action.
    pub timestamp_secs: u64,
    /// The subject (user) who performed the action.
    pub subject: String,
    /// The action performed (e.g. `"repo.create"`, `"ref.force_push"`).
    pub action: String,
    /// The repository this action was performed on (if applicable).
    pub repo_id: Option<RepoId>,
    /// Whether the action was allowed or denied.
    pub outcome: AuditOutcome,
}
