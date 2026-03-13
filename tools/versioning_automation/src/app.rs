//! tools/versioning_automation/src/app.rs
use crate::automation;
use crate::cli_action::{CliAction, parse};
use crate::git;
use crate::issues;
use crate::pr;

pub(crate) fn run(args: Vec<String>) -> i32 {
    run_with(args)
}

pub(crate) fn run_with(args: Vec<String>) -> i32 {
    match parse(&args) {
        Ok(CliAction::ShowHelp(help)) => {
            println!("{help}");
            0
        }
        Ok(CliAction::RunAutomation(passthrough)) => run_automation_native(&passthrough),
        Ok(CliAction::RunGit(passthrough)) => run_git_native(&passthrough),
        Ok(CliAction::RunPr(passthrough)) => run_pr_native(&passthrough),
        Ok(CliAction::RunIssue(passthrough)) => run_issue_native(&passthrough),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}

fn run_pr_native(args: &[String]) -> i32 {
    pr::run(args)
}

fn run_issue_native(args: &[String]) -> i32 {
    issues::run(args)
}

fn run_git_native(args: &[String]) -> i32 {
    git::run(args)
}

fn run_automation_native(args: &[String]) -> i32 {
    automation::run(args)
}
