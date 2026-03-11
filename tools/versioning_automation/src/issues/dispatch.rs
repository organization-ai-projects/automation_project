//! tools/versioning_automation/src/issues/dispatch.rs
use crate::issues::commands::IssueAction;
use crate::issues::execute::{
    run_assignee_logins, run_auto_link, run_close, run_create, run_delete, run_done_status,
    run_fetch_non_compliance_reason, run_field, run_has_label, run_label_exists, run_list_by_label,
    run_neutralize, run_non_compliance_reason, run_open_numbers, run_read, run_reevaluate,
    run_reopen, run_reopen_on_dev, run_repo_name, run_required_fields_validate, run_state,
    run_subissue_refs, run_tasklist_refs, run_update, run_upsert_marker_comment,
};
use crate::issues::parse::parse;
use crate::issues::render::print_usage;
use crate::issues::sync_project_status::run_sync_project_status;

pub(crate) fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(IssueAction::Help) => {
            print_usage();
            0
        }
        Ok(IssueAction::Create(opts)) => run_create(opts),
        Ok(IssueAction::DoneStatus(opts)) => run_done_status(opts),
        Ok(IssueAction::ReopenOnDev(opts)) => run_reopen_on_dev(opts),
        Ok(IssueAction::Read(opts)) => run_read(opts),
        Ok(IssueAction::Update(opts)) => run_update(opts),
        Ok(IssueAction::RepoName) => run_repo_name(),
        Ok(IssueAction::Close(opts)) => run_close(opts),
        Ok(IssueAction::Reopen(opts)) => run_reopen(opts),
        Ok(IssueAction::Delete(opts)) => run_delete(opts),
        Ok(IssueAction::Reevaluate(opts)) => run_reevaluate(opts),
        Ok(IssueAction::Neutralize(opts)) => run_neutralize(opts),
        Ok(IssueAction::AutoLink(opts)) => run_auto_link(opts),
        Ok(IssueAction::RequiredFieldsValidate(opts)) => run_required_fields_validate(opts),
        Ok(IssueAction::NonComplianceReason(opts)) => run_non_compliance_reason(opts),
        Ok(IssueAction::FetchNonComplianceReason(opts)) => run_fetch_non_compliance_reason(opts),
        Ok(IssueAction::LabelExists(opts)) => run_label_exists(opts),
        Ok(IssueAction::SyncProjectStatus(opts)) => run_sync_project_status(opts),
        Ok(IssueAction::TasklistRefs(opts)) => run_tasklist_refs(opts),
        Ok(IssueAction::SubissueRefs(opts)) => run_subissue_refs(opts),
        Ok(IssueAction::UpsertMarkerComment(opts)) => run_upsert_marker_comment(opts),
        Ok(IssueAction::OpenNumbers(opts)) => run_open_numbers(opts),
        Ok(IssueAction::AssigneeLogins(opts)) => run_assignee_logins(opts),
        Ok(IssueAction::State(opts)) => run_state(opts),
        Ok(IssueAction::HasLabel(opts)) => run_has_label(opts),
        Ok(IssueAction::ListByLabel(opts)) => run_list_by_label(opts),
        Ok(IssueAction::Field(opts)) => run_field(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}
