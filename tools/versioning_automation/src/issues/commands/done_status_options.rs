#[derive(Debug, Clone)]
pub(crate) enum DoneStatusMode {
    OnDevMerge,
    OnIssueClosed,
}

#[derive(Debug, Clone)]
pub(crate) struct DoneStatusOptions {
    pub(crate) mode: DoneStatusMode,
    pub(crate) pr: Option<String>,
    pub(crate) issue: Option<String>,
    pub(crate) label: String,
    pub(crate) repo: Option<String>,
}
