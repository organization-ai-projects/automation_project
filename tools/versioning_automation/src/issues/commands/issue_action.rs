//! tools/versioning_automation/src/issues/commands/issue_action.rs
use crate::issues::commands::{
    CloseOptions, CreateOptions, FetchNonComplianceReasonOptions, IssueTarget, LabelExistsOptions,
    NonComplianceReasonOptions, ReadOptions, ReevaluateOptions, RequiredFieldsValidateOptions,
    SyncProjectStatusOptions, UpdateOptions,
};

#[derive(Debug, Clone)]
pub(crate) enum IssueAction {
    Help,
    Create(CreateOptions),
    Read(ReadOptions),
    Update(UpdateOptions),
    Close(CloseOptions),
    Reopen(IssueTarget),
    Delete(IssueTarget),
    Reevaluate(ReevaluateOptions),
    RequiredFieldsValidate(RequiredFieldsValidateOptions),
    NonComplianceReason(NonComplianceReasonOptions),
    FetchNonComplianceReason(FetchNonComplianceReasonOptions),
    LabelExists(LabelExistsOptions),
    SyncProjectStatus(SyncProjectStatusOptions),
}
