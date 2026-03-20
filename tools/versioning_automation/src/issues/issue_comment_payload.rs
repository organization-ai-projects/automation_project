//! tools/versioning_automation/src/issues/issue_comment_payload.rs
use protocol::ProtocolId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct IssueCommentPayload {
    pub(crate) id: ProtocolId,
    pub(crate) body: Option<String>,
    pub(crate) updated_at: Option<String>,
}
