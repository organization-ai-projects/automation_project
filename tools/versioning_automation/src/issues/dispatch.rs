//! tools/versioning_automation/src/issues/dispatch.rs
use crate::issues::commands::{
    AssigneeLoginsOptions, AutoLinkOptions, CloseOptions, ClosureHygieneOptions, CreateOptions,
    DoneStatusOptions, ExtractRefsOptions, FetchNonComplianceReasonOptions, HasLabelOptions,
    IsRootParentOptions, IssueAction, IssueFieldOptions, IssueTarget, LabelExistsOptions,
    ListByLabelOptions, NeutralizeOptions, NonComplianceReasonOptions, OpenNumbersOptions,
    OpenSnapshotsOptions, ParentGuardOptions, ReadOptions, ReevaluateOptions, ReopenOnDevOptions,
    RequiredFieldsValidateOptions, StateOptions, SubissueRefsOptions, TasklistRefsOptions,
    UpdateOptions, UpsertMarkerCommentOptions, ValidateFooterOptions,
};

use crate::issues::{
    parse, print_usage, run_current_login, run_repo_name, run_sync_project_status,
};

pub(crate) fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(IssueAction::Help) => {
            print_usage();
            0
        }
        Ok(IssueAction::Create(opts)) => CreateOptions::run_create(opts),
        Ok(IssueAction::DoneStatus(opts)) => DoneStatusOptions::run_done_status(opts),
        Ok(IssueAction::ReopenOnDev(opts)) => ReopenOnDevOptions::run_reopen_on_dev(opts),
        Ok(IssueAction::Read(opts)) => ReadOptions::run_read(opts),
        Ok(IssueAction::Update(opts)) => UpdateOptions::run_update(opts),
        Ok(IssueAction::RepoName) => run_repo_name(),
        Ok(IssueAction::CurrentLogin) => run_current_login(),
        Ok(IssueAction::IsRootParent(opts)) => IsRootParentOptions::run_is_root_parent(opts),
        Ok(IssueAction::ValidateFooter(opts)) => ValidateFooterOptions::run_validate_footer(opts),
        Ok(IssueAction::Close(opts)) => CloseOptions::run_close(opts),
        Ok(IssueAction::Reopen(opts)) => IssueTarget::run_reopen(opts),
        Ok(IssueAction::Delete(opts)) => IssueTarget::run_delete(opts),
        Ok(IssueAction::Reevaluate(opts)) => ReevaluateOptions::run_reevaluate(opts),
        Ok(IssueAction::Neutralize(opts)) => NeutralizeOptions::run_neutralize(opts),
        Ok(IssueAction::AutoLink(opts)) => AutoLinkOptions::run_auto_link(opts),
        Ok(IssueAction::ParentGuard(opts)) => ParentGuardOptions::run_parent_guard(opts),
        Ok(IssueAction::ClosureHygiene(opts)) => ClosureHygieneOptions::run_closure_hygiene(opts),
        Ok(IssueAction::RequiredFieldsValidate(opts)) => {
            RequiredFieldsValidateOptions::run_required_fields_validate(opts)
        }
        Ok(IssueAction::NonComplianceReason(opts)) => {
            NonComplianceReasonOptions::run_non_compliance_reason(opts)
        }
        Ok(IssueAction::FetchNonComplianceReason(opts)) => {
            FetchNonComplianceReasonOptions::run_fetch_non_compliance_reason(opts)
        }
        Ok(IssueAction::LabelExists(opts)) => LabelExistsOptions::run_label_exists(opts),
        Ok(IssueAction::SyncProjectStatus(opts)) => run_sync_project_status(opts),
        Ok(IssueAction::TasklistRefs(opts)) => TasklistRefsOptions::run_tasklist_refs(opts),
        Ok(IssueAction::SubissueRefs(opts)) => SubissueRefsOptions::run_subissue_refs(opts),
        Ok(IssueAction::UpsertMarkerComment(opts)) => {
            UpsertMarkerCommentOptions::run_upsert_marker_comment(opts)
        }
        Ok(IssueAction::OpenNumbers(opts)) => OpenNumbersOptions::run_open_numbers(opts),
        Ok(IssueAction::OpenSnapshots(opts)) => OpenSnapshotsOptions::run_open_snapshots(opts),
        Ok(IssueAction::ExtractRefs(opts)) => ExtractRefsOptions::run_extract_refs(opts),
        Ok(IssueAction::AssigneeLogins(opts)) => AssigneeLoginsOptions::run_assignee_logins(opts),
        Ok(IssueAction::State(opts)) => StateOptions::run_state(opts),
        Ok(IssueAction::HasLabel(opts)) => HasLabelOptions::run_has_label(opts),
        Ok(IssueAction::ListByLabel(opts)) => ListByLabelOptions::run_list_by_label(opts),
        Ok(IssueAction::Field(opts)) => IssueFieldOptions::run_field(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}
