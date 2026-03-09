//! tools/versioning_automation/src/issues/model/read_options.rs
#[derive(Debug, Clone)]
pub(crate) struct ReadOptions {
    pub(crate) issue: Option<String>,
    pub(crate) repo: Option<String>,
    pub(crate) json: Option<String>,
    pub(crate) jq: Option<String>,
    pub(crate) template: Option<String>,
}
