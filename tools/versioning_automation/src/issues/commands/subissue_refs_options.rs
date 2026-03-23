//! tools/versioning_automation/src/issues/commands/subissue_refs_options.rs
#[derive(Debug, Clone)]
pub(crate) struct SubissueRefsOptions {
    pub(crate) owner: String,
    pub(crate) repo: String,
    pub(crate) issue: String,
}
