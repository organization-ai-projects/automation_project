//! tools/versioning_automation/src/issues/commands/fetch_non_compliance_reason_options.rs
use crate::issues::{Validation, execute::print_string_result};

#[derive(Debug, Clone)]
pub(crate) struct FetchNonComplianceReasonOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}

impl FetchNonComplianceReasonOptions {
    pub(crate) fn run_fetch_non_compliance_reason(self) -> i32 {
        print_string_result(
            Validation::fetch_non_compliance_reason(&self.issue, self.repo.as_deref()),
            1,
        )
    }
}
