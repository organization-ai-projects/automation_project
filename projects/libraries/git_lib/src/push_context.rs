// projects/libraries/git_lib/src/push_context.rs
use command_runner::CommandError;
use std::path::Path;
use std::result::Result as StdResult;

use crate::{commands::push_with_context, repo_context::RepoContext};

/// Context for Git push operations.
pub struct PushContext<'a> {
    pub repo_path: &'a Path,
    pub logs: &'a mut Vec<String>,
    pub remote: &'a str,
    pub branch: Option<&'a str>, // Optional to distinguish cases
}

impl<'a> PushContext<'a> {
    /// Performs a push on the current branch or a specific branch.
    pub fn push(&mut self, set_upstream_if_missing: bool) -> StdResult<(), CommandError> {
        if let Some(branch) = self.branch {
            push_with_context(
                &mut RepoContext {
                    repo_path: self.repo_path,
                    logs: self.logs,
                },
                self.remote,
                Some(branch),
                set_upstream_if_missing,
            )
        } else {
            push_with_context(
                &mut RepoContext {
                    repo_path: self.repo_path,
                    logs: self.logs,
                },
                self.remote,
                None,
                set_upstream_if_missing,
            )
        }
    }
}
