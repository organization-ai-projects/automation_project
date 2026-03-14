//! tools/versioning_automation/src/issues/commands/extract_refs_options.rs
use crate::issues::commands::ExtractRefsProfile;

#[derive(Debug, Clone)]
pub(crate) struct ExtractRefsOptions {
    pub(crate) profile: ExtractRefsProfile,
    pub(crate) text: Option<String>,
    pub(crate) file: Option<String>,
}
