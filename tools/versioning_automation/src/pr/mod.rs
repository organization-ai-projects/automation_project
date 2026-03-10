mod auto_add;
mod closure_marker;
mod closure_refs;
mod commands;
mod conflicts;
mod contracts;
mod domain;
mod issue_decision;
mod non_closing_refs;
mod parse;
mod render;
mod resolve_category;
mod scan;
mod state;

#[cfg(test)]
mod tests;

use auto_add::run_auto_add_closes;
use closure_marker::run_closure_marker;
use closure_refs::run_closure_refs;
use commands::pr_action::PrAction;
use commands::pr_directives_format::PrDirectivesFormat;
use conflicts::run_directive_conflicts;
use issue_decision::run_issue_decision;
use non_closing_refs::run_non_closing_refs;
use parse::parse;
use render::{emit_json, emit_plain, print_usage};
use resolve_category::run_resolve_category;
use scan::scan_directives;
use state::run_directives_state;

pub fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(PrAction::Help) => {
            print_usage();
            0
        }
        Ok(PrAction::Directives(opts)) => {
            let records = scan_directives(&opts.text, opts.unique);
            match opts.format {
                PrDirectivesFormat::Plain => {
                    emit_plain(&records);
                    0
                }
                PrDirectivesFormat::Json => emit_json(&records),
            }
        }
        Ok(PrAction::ClosureRefs(opts)) => run_closure_refs(opts),
        Ok(PrAction::DirectivesState(opts)) => run_directives_state(opts),
        Ok(PrAction::DirectiveConflicts(opts)) => run_directive_conflicts(opts),
        Ok(PrAction::IssueDecision(opts)) => run_issue_decision(opts),
        Ok(PrAction::ClosureMarker(opts)) => run_closure_marker(opts),
        Ok(PrAction::NonClosingRefs(opts)) => run_non_closing_refs(opts),
        Ok(PrAction::ResolveCategory(opts)) => run_resolve_category(opts),
        Ok(PrAction::AutoAddCloses(opts)) => run_auto_add_closes(opts),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}
