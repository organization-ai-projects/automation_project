//! tools/versioning_automation/src/git/commands/clean_local_gone_options.rs
use crate::automation::{
    ensure_git_repo, git_fetch_prune, is_protected_branch, list_gone_branches, run_git,
};

#[derive(Debug)]
pub(crate) struct CleanLocalGoneOptions {
    pub(crate) remote: String,
}

impl CleanLocalGoneOptions {
    pub(crate) fn run_clean_local_gone(self) -> Result<(), String> {
        ensure_git_repo()?;
        git_fetch_prune(&self.remote)?;

        for branch in list_gone_branches()? {
            if is_protected_branch(&branch) {
                continue;
            }
            if run_git(&["branch", "-d", &branch]).is_err() {
                let _ = run_git(&["branch", "-D", &branch]);
            }
        }

        Ok(())
    }
}
