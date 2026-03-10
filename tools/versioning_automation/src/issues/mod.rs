//! tools/versioning_automation/src/issues/mod.rs
mod commands;
mod execute;
mod parse;
mod render;
mod required_fields;

#[cfg(test)]
mod tests;

use commands::IssueAction;
use execute::{
    run_close, run_create, run_delete, run_fetch_non_compliance_reason, run_non_compliance_reason,
    run_read, run_reevaluate, run_reopen, run_required_fields_validate, run_update,
};
use parse::parse;
use render::print_usage;

pub fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(IssueAction::Help) => {
            print_usage();
            0
        }
        Ok(IssueAction::Create(opts)) => run_create(opts),
        Ok(IssueAction::Read(opts)) => run_read(opts),
        Ok(IssueAction::Update(opts)) => run_update(opts),
        Ok(IssueAction::Close(opts)) => run_close(opts),
        Ok(IssueAction::Reopen(opts)) => run_reopen(opts),
        Ok(IssueAction::Delete(opts)) => run_delete(opts),
        Ok(IssueAction::Reevaluate(opts)) => run_reevaluate(opts),
        Ok(IssueAction::RequiredFieldsValidate(opts)) => run_required_fields_validate(opts),
        Ok(IssueAction::NonComplianceReason(opts)) => run_non_compliance_reason(opts),
        Ok(IssueAction::FetchNonComplianceReason(opts)) => run_fetch_non_compliance_reason(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}
