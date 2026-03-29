//! tools/versioning_automation/src/compare_snapshot.rs
use crate::pr::CommitInfo;

#[derive(Debug, Clone)]
pub(crate) struct CompareSnapshot {
    pub(crate) base_ref: String,
    pub(crate) head_ref: String,
    pub(crate) commits: Vec<CommitInfo>,
}

impl CompareSnapshot {
    pub(crate) fn load_compare_snapshot(base_ref: &str, head_ref: &str) -> Result<Self, String> {
        Ok(Self {
            base_ref: base_ref.to_string(),
            head_ref: head_ref.to_string(),
            commits: CommitInfo::compare_api_commits(base_ref, head_ref)?,
        })
    }
}
