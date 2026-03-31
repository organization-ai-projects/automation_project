//! tools/versioning_automation/src/git/commands/create_after_delete_options.rs
use crate::automation::{
    branch_exists_local, branch_exists_remote, current_branch, ensure_git_repo, git_fetch_prune,
    require_non_protected_branch, run_git,
};

#[derive(Debug)]
pub(crate) struct CreateAfterDeleteOptions {
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}

impl CreateAfterDeleteOptions {
    pub(crate) fn run_create_after_delete(self) -> Result<(), String> {
        ensure_git_repo()?;

        let branch_name = current_branch()?;
        require_non_protected_branch(&branch_name)?;

        git_fetch_prune(&self.remote)?;
        run_git(&["checkout", &self.base_branch])?;
        run_git(&["pull", &self.remote, &self.base_branch])?;

        if branch_exists_local(&branch_name) {
            run_git(&["branch", "-d", &branch_name])?;
        }

        if branch_exists_remote(&self.remote, &branch_name) {
            run_git(&["push", &self.remote, "--delete", &branch_name])?;
        }

        run_git(&["checkout", "-b", &branch_name, &self.base_branch])?;
        run_git(&["push", "--set-upstream", &self.remote, &branch_name])?;
        Ok(())
    }
}
