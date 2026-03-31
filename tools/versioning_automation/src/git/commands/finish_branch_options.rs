//! tools/versioning_automation/src/git/commands/finish_branch_options.rs
use crate::automation::{
    branch_exists_local, branch_exists_remote, current_branch, ensure_git_repo, git_fetch_prune,
    require_non_protected_branch, run_git,
};

#[derive(Debug)]
pub(crate) struct FinishBranchOptions {
    pub(crate) branch_name: Option<String>,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}

impl FinishBranchOptions {
    pub(crate) fn run_finish_branch(self) -> Result<(), String> {
        ensure_git_repo()?;

        let branch_name = match self.branch_name {
            Some(name) => name,
            None => current_branch()?,
        };

        require_non_protected_branch(&branch_name)?;
        git_fetch_prune(&self.remote)?;

        let current = current_branch()?;
        if current == branch_name {
            if branch_exists_local(&self.base_branch) {
                run_git(&["checkout", &self.base_branch])?;
            } else {
                let remote_ref = format!("{}/{}", self.remote, self.base_branch);
                run_git(&["checkout", "-b", &self.base_branch, &remote_ref])?;
            }
            run_git(&["pull", &self.remote, &self.base_branch])?;
        }

        if branch_exists_local(&branch_name) && run_git(&["branch", "-d", &branch_name]).is_err() {
            run_git(&["branch", "-D", &branch_name])?;
        }

        if branch_exists_remote(&self.remote, &branch_name) {
            let _ = run_git(&["push", &self.remote, "--delete", &branch_name]);
        }

        git_fetch_prune(&self.remote)?;
        Ok(())
    }
}
