//! tools/versioning_automation/src/issues/commands/parent_guard_options.rs

#[derive(Debug, Clone)]
pub(crate) struct ParentGuardOptions {
    pub(crate) issue: Option<String>,
    pub(crate) child: Option<String>,
    pub(crate) strict_guard: bool,
}
