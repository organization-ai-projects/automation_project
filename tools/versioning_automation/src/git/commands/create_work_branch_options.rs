//! tools/versioning_automation/src/git/commands/create_work_branch_options.rs
use crate::automation::{
    branch_exists_local, branch_exists_remote, ensure_git_repo, git_fetch_prune,
    require_clean_tree, run_git, sanitize_description, validate_branch_name, validate_branch_type,
};

#[derive(Debug)]
pub(crate) struct CreateWorkBranchOptions {
    pub(crate) branch_type: String,
    pub(crate) description: String,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}

impl CreateWorkBranchOptions {
    pub(crate) fn run_create_work_branch(self) -> Result<(), String> {
        ensure_git_repo()?;
        validate_branch_type(&self.branch_type)?;
        require_clean_tree()?;

        let description = sanitize_description(&self.description);
        if description.is_empty() {
            return Err("Description cannot be empty after sanitization.".to_string());
        }

        let branch_name = format!("{}/{}", self.branch_type, description);
        validate_branch_name(&branch_name)?;

        git_fetch_prune(&self.remote)?;
        if branch_exists_local(&branch_name) {
            return Err(format!("Branch '{branch_name}' already exists locally."));
        }
        if branch_exists_remote(&self.remote, &branch_name) {
            return Err(format!(
                "Branch '{branch_name}' already exists on remote {}.",
                self.remote
            ));
        }

        if branch_exists_local(&self.base_branch) {
            run_git(&["checkout", &self.base_branch])?;
        } else {
            let remote_ref = format!("{}/{}", self.remote, self.base_branch);
            run_git(&["checkout", "-b", &self.base_branch, &remote_ref])?;
        }

        run_git(&["pull", &self.remote, &self.base_branch])?;
        run_git(&["checkout", "-b", &branch_name, &self.base_branch])?;
        run_git(&["push", "--set-upstream", &self.remote, &branch_name])?;
        Ok(())
    }
}
