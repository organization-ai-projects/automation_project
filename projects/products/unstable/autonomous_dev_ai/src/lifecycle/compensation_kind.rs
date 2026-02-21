//projects/products/unstable/autonomous_dev_ai/src/lifecycle/compensation_kind.rs
use serde::{Deserialize, Serialize};

/// Describes whether an action can be undone and the compensation strategy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompensationKind {
    /// No rollback needed (read-only or idempotent).
    None,
    /// Can be reversed programmatically (e.g., delete the created branch).
    Reversible { description: String },
    /// Cannot be reversed; requires manual intervention.
    Irreversible { warning: String },
}
