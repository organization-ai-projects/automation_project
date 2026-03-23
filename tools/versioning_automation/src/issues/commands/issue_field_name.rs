//! tools/versioning_automation/src/issues/commands/issue_field_name.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IssueFieldName {
    Title,
    Body,
    LabelsRaw,
}
