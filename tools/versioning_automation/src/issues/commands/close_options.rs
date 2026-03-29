//! tools/versioning_automation/src/issues/commands/close_options.rs
use crate::gh_cli::{add_repo_arg, gh_command, push_arg, status_code_owned};

#[derive(Debug, Clone)]
pub(crate) struct CloseOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
    pub(crate) reason: String,
    pub(crate) comment: Option<String>,
}

impl CloseOptions {
    pub(crate) fn run_close(self) -> i32 {
        let mut cmd = gh_command(&["issue", "close", &self.issue, "--reason", &self.reason]);
        add_repo_arg(&mut cmd, self.repo.as_deref());
        if let Some(comment) = &self.comment {
            push_arg(&mut cmd, "--comment");
            push_arg(&mut cmd, comment);
        }
        let status = status_code_owned(cmd);
        if status == 0 {
            println!("Issue #{} closed (reason: {}).", self.issue, self.reason);
        }
        status
    }
}
