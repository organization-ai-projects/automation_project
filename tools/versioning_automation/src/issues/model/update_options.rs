//! tools/versioning_automation/src/issues/model/update_options.rs
#[derive(Debug, Clone)]
pub(crate) struct UpdateOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
    pub(crate) edit_args: Vec<(String, String)>,
}
