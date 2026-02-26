use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepoVersioningDelta {
    pub before_head_commit: Option<String>,
    pub after_head_commit: Option<String>,
    pub touched_files: Vec<String>,
    pub worktree_changed: bool,
}
