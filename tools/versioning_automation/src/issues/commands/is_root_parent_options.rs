//! tools/versioning_automation/src/issues/commands/is_root_parent_options.rs
#[derive(Debug, Clone)]
pub(crate) struct IsRootParentOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}
