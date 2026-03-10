//! tools/versioning_automation/src/issues/dispatch.rs

use crate::issues::commands::IssueAction;
use crate::issues::execute::{
    run_close, run_create, run_delete, run_fetch_non_compliance_reason, run_label_exists,
    run_non_compliance_reason, run_read, run_reevaluate, run_reopen, run_repo_name,
    run_required_fields_validate, run_update,
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
        Ok(IssueAction::Read(opts)) => run_read(opts),
        Ok(IssueAction::Update(opts)) => run_update(opts),
        Ok(IssueAction::RepoName) => run_repo_name(),
        Ok(IssueAction::Close(opts)) => run_close(opts),
        Ok(IssueAction::Reopen(opts)) => run_reopen(opts),
        Ok(IssueAction::Delete(opts)) => run_delete(opts),
        Ok(IssueAction::Reevaluate(opts)) => run_reevaluate(opts),
        Ok(IssueAction::RequiredFieldsValidate(opts)) => run_required_fields_validate(opts),
        Ok(IssueAction::NonComplianceReason(opts)) => run_non_compliance_reason(opts),
        Ok(IssueAction::FetchNonComplianceReason(opts)) => run_fetch_non_compliance_reason(opts),
        Ok(IssueAction::LabelExists(opts)) => run_label_exists(opts),
        Ok(IssueAction::SyncProjectStatus(opts)) => run_sync_project_status(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}
