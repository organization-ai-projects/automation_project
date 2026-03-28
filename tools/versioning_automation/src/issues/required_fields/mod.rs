//! tools/versioning_automation/src/issues/required_fields/mod.rs
mod contract_values;
mod gh_issue_label;
mod gh_issue_payload;
mod key;
mod labels;
mod parser;
mod validation;

#[cfg(test)]
mod tests;

pub(crate) use contract_values::ContractValues;
pub(crate) use gh_issue_payload::GhIssuePayload;
pub(crate) use key::Key;
pub(crate) use parser::{body_has_section, extract_field_value, trim_whitespace};
pub(crate) use validation::Validation;
