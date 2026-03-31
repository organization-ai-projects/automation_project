//! tools/versioning_automation/src/git/commands/add_commit_push_options.rs
use crate::{
    automation::{
        current_branch, ensure_git_repo, require_non_protected_branch, run_git, run_git_output,
        validate_commit_message,
    },
    git::commands::PushBranchOptions,
};

#[derive(Debug)]
pub(crate) struct AddCommitPushOptions {
    pub(crate) message: String,
    pub(crate) no_verify: bool,
    pub(crate) remote: String,
}

impl AddCommitPushOptions {
    pub(crate) fn run_add_commit_push(self) -> Result<(), String> {
        ensure_git_repo()?;
        validate_commit_message(&self.message)?;

        let branch_name = current_branch()?;
        require_non_protected_branch(&branch_name)?;

        run_git(&["add", "-A"])?;

        let staged_files = run_git_output(&["diff", "--cached", "--name-only"])?;
        if staged_files.trim().is_empty() {
            return Ok(());
        }

        if self.no_verify {
            run_git(&["commit", "--no-verify", "-m", &self.message])?;
        } else {
            run_git(&["commit", "-m", &self.message])?;
        }

        PushBranchOptions::run_push_branch(PushBranchOptions {
            remote: self.remote,
        })
    }
}
