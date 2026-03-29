//! tools/versioning_automation/src/issues/commands/update_options.rs
use crate::gh_cli::{gh_issue_target_command, push_arg, status_code_owned};

#[derive(Debug, Clone)]
pub(crate) struct UpdateOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
    pub(crate) edit_args: Vec<(String, String)>,
}

impl UpdateOptions {
    pub(crate) fn run_update(self) -> i32 {
        let mut cmd = gh_issue_target_command("edit", &self.issue, self.repo.as_deref());
        for (flag, value) in &self.edit_args {
            push_arg(&mut cmd, flag);
            push_arg(&mut cmd, value);
        }
        let status = status_code_owned(cmd);
        if status == 0 {
            println!("Issue #{} updated.", self.issue);
        }
        status
    }
}
