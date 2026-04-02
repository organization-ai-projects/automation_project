//! tools/versioning_automation/src/issues/commands/open_snapshots_options.rs
#[derive(Debug, Clone)]
pub(crate) struct OpenSnapshotsOptions {
    pub(crate) repo: Option<String>,
    pub(crate) limit: usize,
}
