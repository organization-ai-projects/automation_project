//! tools/versioning_automation/src/issues/commands/upsert_marker_comment_options.rs
use crate::{
    gh_cli::output_trim_or_empty, issue_comment_upsert::upsert_issue_comment_by_marker,
    issues::IssueCommentPayload,
};

#[derive(Debug, Clone)]
pub(crate) struct UpsertMarkerCommentOptions {
    pub(crate) repo: String,
    pub(crate) issue: String,
    pub(crate) marker: String,
    pub(crate) body: String,
    pub(crate) announce: bool,
}

impl UpsertMarkerCommentOptions {
    pub(crate) fn run_upsert_marker_comment(self) -> i32 {
        let comments_endpoint = format!("repos/{}/issues/{}/comments", self.repo, self.issue);
        let comments_json = output_trim_or_empty(&["api", &comments_endpoint]);
        let comments = IssueCommentPayload::parse_issue_comments(&comments_json);
        let had_existing_comment =
            IssueCommentPayload::find_latest_matching_comment_id(&comments, &self.marker).is_some();
        let status =
            match upsert_issue_comment_by_marker(&self.repo, &self.issue, &self.marker, &self.body)
            {
                Ok(_) => 0,
                Err(err) => {
                    eprintln!("{err}");
                    1
                }
            };

        if status != 0 {
            return status;
        }

        if self.announce {
            if had_existing_comment {
                println!("Updated parent status comment on #{}.", self.issue);
            } else {
                println!("Posted parent status comment on #{}.", self.issue);
            }
        }

        0
    }
}
