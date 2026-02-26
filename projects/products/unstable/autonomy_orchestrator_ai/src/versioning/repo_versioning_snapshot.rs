use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoVersioningSnapshot {
    pub head_commit: Option<String>,
    pub status_porcelain: String,
    pub changed_files: Vec<String>,
}
