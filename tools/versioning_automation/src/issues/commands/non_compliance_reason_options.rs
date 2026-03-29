//! tools/versioning_automation/src/issues/commands/non_compliance_reason_options.rs
use crate::issues::{Validation, execute::print_string_result};

#[derive(Debug, Clone)]
pub(crate) struct NonComplianceReasonOptions {
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) labels_raw: String,
}

impl NonComplianceReasonOptions {
    pub(crate) fn run_non_compliance_reason(self) -> i32 {
        print_string_result(
            Validation::non_compliance_reason_from_content(
                &self.title,
                &self.body,
                &self.labels_raw,
            ),
            1,
        )
    }
}
