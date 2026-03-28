//! tools/versioning_automation/src/issues/commands/done_status_mode.rs
#[derive(Debug, Clone)]
pub(crate) enum DoneStatusMode {
    OnDevMerge,
    OnIssueClosed,
}
