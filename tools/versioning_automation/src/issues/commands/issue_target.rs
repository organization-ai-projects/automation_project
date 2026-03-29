//! tools/versioning_automation/src/issues/commands/issue_target.rs
use crate::gh_cli::{add_repo_arg, gh_command, gh_issue_target_command, status_code_owned};

#[derive(Debug, Clone)]
pub(crate) struct IssueTarget {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}

impl IssueTarget {
    pub(crate) fn run_reopen(self) -> i32 {
        let cmd = gh_issue_target_command("reopen", &self.issue, self.repo.as_deref());
        let status = status_code_owned(cmd);
        if status == 0 {
            println!("Issue #{} reopened.", self.issue);
        }
        status
    }

    pub(crate) fn run_delete(self) -> i32 {
        let mut cmd = gh_command(&["issue", "close", &self.issue, "--reason", "not_planned"]);
        add_repo_arg(&mut cmd, self.repo.as_deref());
        let status = status_code_owned(cmd);
        if status == 0 {
            println!(
                "Issue #{} soft-deleted (closed with reason: not_planned).",
                self.issue
            );
        }
        status
    }
}
