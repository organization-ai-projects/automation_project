//! tools/versioning_automation/src/git/commands/clean_branches_options.rs
use crate::automation::{
    ensure_git_repo, get_merged_branches, git_fetch_prune, is_protected_branch, list_gone_branches,
    run_git,
};

#[derive(Debug)]
pub(crate) struct CleanBranchesOptions {
    pub(crate) dry_run: bool,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}

impl CleanBranchesOptions {
    pub(crate) fn run_clean_branches(self) -> Result<(), String> {
        ensure_git_repo()?;
        git_fetch_prune(&self.remote)?;

        for branch in list_gone_branches()? {
            if is_protected_branch(&branch) {
                continue;
            }

            if self.dry_run {
                println!("[DRY-RUN] Would delete local branch: {branch}");
                continue;
            }

            if run_git(&["branch", "-d", &branch]).is_err() {
                let _ = run_git(&["branch", "-D", &branch]);
            }
        }

        let merged_branches = get_merged_branches(&self.base_branch)?;
        for branch in merged_branches {
            if branch.is_empty() || is_protected_branch(&branch) {
                continue;
            }
            println!("{branch}");
        }

        Ok(())
    }
}
