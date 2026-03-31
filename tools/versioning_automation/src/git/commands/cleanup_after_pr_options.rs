//! tools/versioning_automation/src/git/commands/cleanup_after_pr_options.rs
use crate::automation::{
    branch_exists_local, branch_exists_remote, current_branch, ensure_git_repo, git_fetch_prune,
    is_protected_branch, run_git, run_git_output,
};

#[derive(Debug)]
pub(crate) struct CleanupAfterPrOptions {
    pub(crate) delete_only: bool,
    pub(crate) remote: String,
    pub(crate) base_branch: String,
}

impl CleanupAfterPrOptions {
    pub(crate) fn run_cleanup_after_pr(self) -> Result<(), String> {
        ensure_git_repo()?;

        let current_branch = current_branch().ok();

        run_git(&["checkout", &self.base_branch])?;
        run_git(&["pull", &self.remote, &self.base_branch])?;
        git_fetch_prune(&self.remote)?;

        let locals = run_git_output(&["for-each-ref", "--format=%(refname:short)", "refs/heads"])?;

        let mut outdated = Vec::new();
        for branch in locals.lines() {
            if branch.is_empty() || is_protected_branch(branch) {
                continue;
            }

            let range = format!("{branch}..{}", self.base_branch);
            let behind_count = run_git_output(&["rev-list", "--count", &range])
                .ok()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(0);

            if behind_count > 0 {
                outdated.push(branch.to_string());
            }
        }

        for branch in outdated {
            if run_git(&["branch", "-d", &branch]).is_err() {
                let _ = run_git(&["branch", "-D", &branch]);
            }

            if branch_exists_remote(&self.remote, &branch) {
                let _ = run_git(&["push", &self.remote, "--delete", &branch]);
            }

            if !self.delete_only {
                run_git(&["checkout", "-b", &branch, &self.base_branch])?;
                run_git(&["push", "--set-upstream", &self.remote, &branch])?;
                run_git(&["checkout", &self.base_branch])?;
            }
        }

        if let Some(branch) = current_branch
            && branch_exists_local(&branch)
        {
            let _ = run_git(&["checkout", &branch]);
        }

        Ok(())
    }
}
