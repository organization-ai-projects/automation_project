// projects/products/stable/platform_versioning/backend/src/sync/ref_update_policy.rs
use serde::{Deserialize, Serialize};

/// Policy that governs how a ref update is validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RefUpdatePolicy {
    /// Only fast-forward updates are accepted (the default).
    #[default]
    FastForwardOnly,
    /// Force updates are allowed (overwrites non-fast-forward; requires `Admin` permission).
    Force,
}
