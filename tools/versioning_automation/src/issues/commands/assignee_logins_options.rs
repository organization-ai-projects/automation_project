//! tools/versioning_automation/src/issues/commands/assignee_logins_options.rs
use crate::{gh_cli::output_trim_or_empty, issues::execute::print_non_empty_lines};

#[derive(Debug, Clone)]
pub(crate) struct AssigneeLoginsOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}

impl AssigneeLoginsOptions {
    pub(crate) fn run_assignee_logins(self) -> i32 {
        let mut args: Vec<&str> = vec![
            "issue",
            "view",
            &self.issue,
            "--json",
            "assignees",
            "--jq",
            ".assignees[].login",
        ];
        if let Some(repo) = self.repo.as_deref() {
            args.push("-R");
            args.push(repo);
        }
        print_non_empty_lines(&output_trim_or_empty(&args));
        0
    }
}
