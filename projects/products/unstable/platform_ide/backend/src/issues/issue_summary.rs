// projects/products/unstable/platform_ide/backend/src/issues/issue_summary.rs
use serde::{Deserialize, Serialize};

/// A summary of a platform issue visible to the current user.
///
/// In the platform model an "issue" maps to a repository that the user's token
/// grants read access to. The IDE presents only the subset that the platform
/// explicitly exposes for the authenticated session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueSummary {
    /// The opaque issue/repository identifier.
    pub id: String,
    /// A human-readable display name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Creation timestamp (Unix seconds).
    pub created_at: u64,
}
