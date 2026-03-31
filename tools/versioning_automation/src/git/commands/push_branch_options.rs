//! tools/versioning_automation/src/git/commands/push_branch_options.rs
use crate::automation::{
    current_branch, ensure_git_repo, git_fetch_prune, has_upstream, require_non_protected_branch,
    run_git,
};

#[derive(Debug)]
pub(crate) struct PushBranchOptions {
    pub(crate) remote: String,
}

impl PushBranchOptions {
    pub(crate) fn run_push_branch(self) -> Result<(), String> {
        ensure_git_repo()?;
        let branch_name = current_branch()?;
        require_non_protected_branch(&branch_name)?;

        git_fetch_prune(&self.remote)?;

        if has_upstream() {
            run_git(&["push", &self.remote, &branch_name])?;
        } else {
            run_git(&["push", "--set-upstream", &self.remote, &branch_name])?;
        }

        Ok(())
    }
}
