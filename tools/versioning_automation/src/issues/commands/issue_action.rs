//! tools/versioning_automation/src/issues/commands/issue_action.rs
use crate::issues::commands::{
    AssigneeLoginsOptions, CloseOptions, CreateOptions, FetchNonComplianceReasonOptions,
    IssueTarget, LabelExistsOptions, NonComplianceReasonOptions, OpenNumbersOptions, ReadOptions,
    ReevaluateOptions, RequiredFieldsValidateOptions, SubissueRefsOptions,
    SyncProjectStatusOptions, TasklistRefsOptions, UpdateOptions, UpsertMarkerCommentOptions,
};

#[derive(Debug, Clone)]
pub(crate) enum IssueAction {
    Help,
    Create(CreateOptions),
    Read(ReadOptions),
    Update(UpdateOptions),
    RepoName,
    Close(CloseOptions),
    Reopen(IssueTarget),
    Delete(IssueTarget),
    Reevaluate(ReevaluateOptions),
    RequiredFieldsValidate(RequiredFieldsValidateOptions),
    NonComplianceReason(NonComplianceReasonOptions),
    FetchNonComplianceReason(FetchNonComplianceReasonOptions),
    LabelExists(LabelExistsOptions),
    SyncProjectStatus(SyncProjectStatusOptions),
    TasklistRefs(TasklistRefsOptions),
    SubissueRefs(SubissueRefsOptions),
    UpsertMarkerComment(UpsertMarkerCommentOptions),
    OpenNumbers(OpenNumbersOptions),
    AssigneeLogins(AssigneeLoginsOptions),
}
