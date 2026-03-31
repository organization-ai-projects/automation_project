//! tools/versioning_automation/src/automation/commands/pre_branch_create_check_options.rs
use crate::{
    automation::execute::{branch_exists_local, run_git_output},
    lazy_regex::BRANCH_NAME_REGEX,
};

#[derive(Debug)]
pub(crate) struct PreBranchCreateCheckOptions {
    pub(crate) branch: String,
}

impl PreBranchCreateCheckOptions {
    pub(crate) fn run_pre_branch_create_check(self) -> Result<(), String> {
        let branch = self.branch.trim();
        if branch.is_empty() {
            return Err("No branch name provided.".to_string());
        }
        let re = BRANCH_NAME_REGEX
            .as_ref()
            .map_err(|e| format!("Regex error: {e}"))?;
        if !re.is_match(branch) {
            return Err(format!(
                "Invalid branch name '{branch}'. Expected (feature|fix|hotfix|release)/<name>"
            ));
        }
        if branch_exists_local(branch) {
            return Err(format!("Branch '{branch}' already exists locally."));
        }
        let marker = format!("[{branch}]");
        let worktrees = run_git_output(&["worktree", "list"])?;
        if worktrees.lines().any(|line| line.contains(&marker)) {
            return Err(format!(
                "Branch '{branch}' is already in use by another worktree."
            ));
        }
        println!("Branch name '{branch}' is valid.");
        Ok(())
    }
}
