// projects/products/unstable/platform_versioning/ui/src/repo_detail_view.rs
use serde::{Deserialize, Serialize};

use crate::ref_entry::RefEntry;

/// The repository detail view state.
///
/// Shows refs and recent history for a specific repository.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RepoDetailView {
    /// The repository id being viewed.
    pub repo_id: Option<String>,
    /// The refs in this repository.
    pub refs: Vec<RefEntry>,
    /// Recent commit messages for display.
    pub recent_commits: Vec<String>,
}

impl RepoDetailView {
    /// Sets the active repository and resets state.
    pub fn open_repo(&mut self, repo_id: String) {
        self.repo_id = Some(repo_id);
        self.refs = vec![];
        self.recent_commits = vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_repo_resets_state() {
        let mut view = RepoDetailView::default();
        view.refs = vec![RefEntry {
            name: "heads/main".to_string(),
            commit_id: "abc".to_string(),
        }];
        view.open_repo("new-repo".to_string());
        assert_eq!(view.repo_id.as_deref(), Some("new-repo"));
        assert!(view.refs.is_empty());
    }
}
