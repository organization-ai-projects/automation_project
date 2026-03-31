//! tools/versioning_automation/src/git/commands/create_branch_options.rs
use crate::automation::{
    branch_exists_local, ensure_git_repo, git_fetch_prune, load_last_deleted_branch,
    require_non_protected_branch, run_git, validate_branch_name,
};

#[derive(Debug)]
pub(crate) struct CreateBranchOptions {
    pub(crate) branch_name: Option<String>,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}

impl CreateBranchOptions {
    pub(crate) fn run_create_branch(self) -> Result<(), String> {
        ensure_git_repo()?;
        let mut branch_name = self
            .branch_name
            .or_else(load_last_deleted_branch)
            .ok_or_else(|| "Missing branch name and no last deleted branch found.".to_string())?;
        branch_name = branch_name.trim().to_string();

        validate_branch_name(&branch_name)?;
        require_non_protected_branch(&branch_name)?;

        git_fetch_prune(&self.remote)?;
        run_git(&["checkout", &self.base_branch])?;
        run_git(&["pull", &self.remote, &self.base_branch])?;

        if branch_exists_local(&branch_name) {
            run_git(&["checkout", &branch_name])?;
        } else {
            run_git(&["checkout", "-b", &branch_name, &self.base_branch])?;
        }

        run_git(&["push", "--set-upstream", &self.remote, &branch_name])?;
        Ok(())
    }
}
