//! tools/versioning_automation/src/issues/commands/create_options.rs
#[derive(Debug, Clone)]
pub(crate) struct CreateOptions {
    pub(crate) title: String,
    pub(crate) context: String,
    pub(crate) problem: String,
    pub(crate) acceptances: Vec<String>,
    pub(crate) parent: String,
    pub(crate) labels: Vec<String>,
    pub(crate) repo: Option<String>,
    pub(crate) dry_run: bool,
}
