//! tools/versioning_automation/src/pr/commands/pr_upsert_comment_options.rs
use crate::{issue_comment_upsert::upsert_issue_comment_by_marker, repo_name::resolve_repo_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrUpsertCommentOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
    pub(crate) marker: String,
    pub(crate) body: String,
}

impl PrUpsertCommentOptions {
    pub(crate) fn run_upsert_comment(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo) {
            Ok(repo) => repo,
            Err(msg) => {
                eprintln!("{msg}");
                return 3;
            }
        };

        match upsert_issue_comment_by_marker(&repo_name, &self.pr_number, &self.marker, &self.body)
        {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("{err}");
                1
            }
        }
    }
}
