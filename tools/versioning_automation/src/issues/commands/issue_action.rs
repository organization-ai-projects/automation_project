//! tools/versioning_automation/src/issues/commands/issue_action.rs
use crate::issues::commands::{
    AssigneeLoginsOptions, CloseOptions, CreateOptions, DoneStatusOptions,
    FetchNonComplianceReasonOptions, HasLabelOptions, IssueFieldOptions, IssueTarget,
    LabelExistsOptions, ListByLabelOptions, NonComplianceReasonOptions, OpenNumbersOptions,
    ReadOptions, ReevaluateOptions, RequiredFieldsValidateOptions, StateOptions,
    SubissueRefsOptions, SyncProjectStatusOptions, TasklistRefsOptions, UpdateOptions,
    UpsertMarkerCommentOptions,
};

#[derive(Debug, Clone)]
pub(crate) enum IssueAction {
    Help,
    Create(CreateOptions),
    Read(ReadOptions),
    DoneStatus(DoneStatusOptions),
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
    State(StateOptions),
    HasLabel(HasLabelOptions),
    ListByLabel(ListByLabelOptions),
    Field(IssueFieldOptions),
}
