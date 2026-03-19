//! tools/versioning_automation/src/issues/commands/sync_project_status_options.rs

#[derive(Debug, Clone)]
pub(crate) struct SyncProjectStatusOptions {
    pub(crate) repo: String,
    pub(crate) issue: String,
    pub(crate) status: String,
}
