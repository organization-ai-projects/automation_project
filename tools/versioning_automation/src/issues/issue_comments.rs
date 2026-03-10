//! tools/versioning_automation/src/issues/issue_comments.rs
use crate::issues::issue_comment_payload::IssueCommentPayload;

pub(crate) fn parse_issue_comments(json: &str) -> Vec<IssueCommentPayload> {
    if json.trim().is_empty() {
        return Vec::new();
    }
    common_json::from_json_str::<Vec<IssueCommentPayload>>(json).unwrap_or_default()
}

pub(crate) fn find_latest_matching_comment_id(
    comments: &[IssueCommentPayload],
    marker: &str,
) -> Option<String> {
    comments
        .iter()
        .filter(|comment| {
            comment
                .body
                .as_deref()
                .is_some_and(|body| body.contains(marker))
        })
        .max_by_key(|comment| comment.updated_at.as_deref().unwrap_or(""))
        .map(|comment| comment.id.to_string())
}
