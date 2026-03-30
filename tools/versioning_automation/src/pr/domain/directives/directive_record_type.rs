//! tools/versioning_automation/src/pr/domain/directives/directive_record_type.rs
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum DirectiveRecordType {
    Event,
    Decision,
    Duplicate,
}
