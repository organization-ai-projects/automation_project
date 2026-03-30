//! tools/versioning_automation/src/issues/issue_comment_payload.rs
use protocol::ProtocolId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct IssueCommentPayload {
    pub(crate) id: ProtocolId,
    pub(crate) body: Option<String>,
    pub(crate) updated_at: Option<String>,
}

impl IssueCommentPayload {
    pub(crate) fn parse_issue_comments(json: &str) -> Vec<Self> {
        if json.trim().is_empty() {
            return Vec::new();
        }
        common_json::from_json_str::<Vec<Self>>(json).unwrap_or_default()
    }

    pub(crate) fn find_latest_matching_comment_id(
        comments: &[Self],
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
}
