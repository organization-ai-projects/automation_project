// projects/products/unstable/platform_versioning/ui/src/repo_list_view.rs
use serde::{Deserialize, Serialize};

use crate::repo_summary::RepoSummary;

/// The repository list view state.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RepoListView {
    /// The currently loaded list of repositories.
    pub repos: Vec<RepoSummary>,
    /// Whether a fetch is in progress.
    pub loading: bool,
}

impl RepoListView {
    /// Updates the list with fresh data from the backend.
    pub fn set_repos(&mut self, repos: Vec<RepoSummary>) {
        self.repos = repos;
        self.loading = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_repos_updates_list() {
        let mut view = RepoListView {
            loading: true,
            ..RepoListView::default()
        };
        view.set_repos(vec![RepoSummary {
            id: "my-repo".to_string(),
            name: "My Repo".to_string(),
            description: None,
        }]);
        assert_eq!(view.repos.len(), 1);
        assert!(!view.loading);
    }
}
