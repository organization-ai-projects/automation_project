//! tools/versioning_automation/src/issues/commands/upsert_marker_comment_options.rs
#[derive(Debug, Clone)]
pub(crate) struct UpsertMarkerCommentOptions {
    pub(crate) repo: String,
    pub(crate) issue: String,
    pub(crate) marker: String,
    pub(crate) body: String,
    pub(crate) announce: bool,
}
