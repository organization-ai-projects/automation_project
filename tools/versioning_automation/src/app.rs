//! tools/versioning_automation/src/app.rs
use crate::cli_action::{CliAction, parse};
use crate::issues;

pub fn run(args: Vec<String>) -> i32 {
    run_with(args)
}

pub(crate) fn run_with(args: Vec<String>) -> i32 {
    match parse(&args) {
        Ok(CliAction::ShowHelp(help)) => {
            println!("{help}");
            0
        }
        Ok(CliAction::RunPr(passthrough)) => run_pr_native(&passthrough),
        Ok(CliAction::RunIssue(passthrough)) => run_issue_native(&passthrough),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}

fn run_pr_native(args: &[String]) -> i32 {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("va pr [args...]");
        println!("Native PR engine migration is in progress.");
        return 0;
    }
    eprintln!("va pr: native engine migration in progress; command path scaffolded.");
    3
}

fn run_issue_native(args: &[String]) -> i32 {
    issues::run(args)
}
