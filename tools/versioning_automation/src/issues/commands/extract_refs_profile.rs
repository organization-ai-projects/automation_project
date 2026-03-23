//! tools/versioning_automation/src/issues/commands/extract_refs_profile.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExtractRefsProfile {
    Hook,
    Audit,
}
