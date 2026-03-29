//! tools/versioning_automation/src/pr/commands/pr_open_referencing_issue_options.rs
use crate::{
    open_pr_issue_refs::load_open_pr_numbers_referencing_issue, repo_name::resolve_repo_name,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrOpenReferencingIssueOptions {
    pub(crate) issue_number: String,
    pub(crate) repo: Option<String>,
}

impl PrOpenReferencingIssueOptions {
    pub(crate) fn run_open_referencing_issue(self) -> i32 {
        let Ok(repo_name) = resolve_repo_name(self.repo) else {
            return 0;
        };

        let matched = load_open_pr_numbers_referencing_issue(&self.issue_number, &repo_name)
            .unwrap_or_default();
        for pr_number in matched {
            println!("{pr_number}");
        }
        0
    }
}
