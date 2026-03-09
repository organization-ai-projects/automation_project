//! tools/versioning_automation/src/issues/mod.rs
mod execute;
mod model;
mod parse;
mod render;

#[cfg(test)]
mod tests;

use execute::{run_close, run_create, run_delete, run_read, run_reopen, run_update};
use model::IssueAction;
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
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}
