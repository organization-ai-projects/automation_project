#[derive(Debug, Clone)]
pub(crate) struct ChangedCratesOptions {
    pub(crate) ref1: Option<String>,
    pub(crate) ref2: Option<String>,
    pub(crate) output_format: Option<String>,
}
