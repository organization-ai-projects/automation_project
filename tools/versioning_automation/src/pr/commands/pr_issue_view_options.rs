//! tools/versioning_automation/src/pr/commands/pr_issue_view_options.rs
use common_json::to_string;

use crate::{issue_remote_snapshot::IssueRemoteSnapshot, repo_name::resolve_repo_name_optional};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrIssueViewOptions {
    pub(crate) issue_number: String,
    pub(crate) repo: Option<String>,
}

impl PrIssueViewOptions {
    pub(crate) fn run_issue_view(self) -> i32 {
        let repo = resolve_repo_name_optional(self.repo.as_deref());
        if let Ok(snapshot) =
            IssueRemoteSnapshot::load_issue_remote_snapshot(&self.issue_number, repo.as_deref())
            && let Ok(json) = to_string(&snapshot)
            && !json.is_empty()
        {
            println!("{json}");
        }
        0
    }
}
