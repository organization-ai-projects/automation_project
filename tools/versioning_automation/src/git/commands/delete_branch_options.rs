//! tools/versioning_automation/src/git/commands/delete_branch_options.rs
use crate::automation::{
    branch_exists_local, branch_exists_remote, current_branch, ensure_git_repo, git_fetch_prune,
    require_non_protected_branch, run_git, save_last_deleted_branch,
};

#[derive(Debug)]
pub(crate) struct DeleteBranchOptions {
    pub(crate) branch_name: String,
    pub(crate) force: bool,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}

impl DeleteBranchOptions {
    pub(crate) fn run_delete_branch(self) -> Result<(), String> {
        ensure_git_repo()?;

        let branch_name = self.branch_name.trim();
        if branch_name.is_empty() {
            return Err("Branch name cannot be empty.".to_string());
        }
        require_non_protected_branch(branch_name)?;

        let current = current_branch()?;
        if current == branch_name {
            run_git(&["checkout", &self.base_branch])?;
        }

        save_last_deleted_branch(branch_name)?;
        git_fetch_prune(&self.remote)?;

        if branch_exists_local(branch_name) {
            if self.force {
                run_git(&["branch", "-D", branch_name])?;
            } else {
                run_git(&["branch", "-d", branch_name])?;
            }
        }

        if branch_exists_remote(&self.remote, branch_name) {
            run_git(&["push", &self.remote, "--delete", branch_name])?;
        }

        Ok(())
    }
}
