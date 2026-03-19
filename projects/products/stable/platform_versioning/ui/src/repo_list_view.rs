// projects/products/stable/platform_versioning/ui/src/repo_list_view.rs
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
