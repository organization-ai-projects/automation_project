//! tools/versioning_automation/src/issues/commands/extract_refs_options.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExtractRefsProfile {
    Hook,
    Audit,
}

#[derive(Debug, Clone)]
pub(crate) struct ExtractRefsOptions {
    pub(crate) profile: ExtractRefsProfile,
    pub(crate) text: Option<String>,
    pub(crate) file: Option<String>,
}
