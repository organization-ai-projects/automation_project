//! tools/versioning_automation/src/app.rs
use crate::automation;
use crate::cli_action::{CliAction, parse};
use crate::git;
use crate::issues;
use crate::pr;
use std::path::Path;

pub(crate) fn run(args: Vec<String>) -> i32 {
    run_with(args)
}

pub(crate) fn run_with(args: Vec<String>) -> i32 {
    if let Some(code) = run_git_hook_entrypoint(&args) {
        return code;
    }

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

fn run_git_hook_entrypoint(args: &[String]) -> Option<i32> {
    let argv0 = args.first()?;
    let hook_name = Path::new(argv0).file_name()?.to_string_lossy();

    match hook_name.as_ref() {
        "pre-commit" => Some(run_automation_native(&["pre-commit-check".to_string()])),
        "pre-push" => Some(run_automation_native(&["pre-push-check".to_string()])),
        "post-checkout" => {
            let is_branch_checkout = args.get(3).map(String::as_str).unwrap_or("0") == "1";
            if !is_branch_checkout {
                return Some(0);
            }
            if std::env::var("SKIP_POST_CHECKOUT_CONVENTION_WARN").unwrap_or_default() == "1" {
                return Some(0);
            }
            let _ = run_automation_native(&["post-checkout-check".to_string()]);
            Some(0)
        }
        "commit-msg" => {
            let Some(file) = args.get(1).cloned() else {
                return Some(2);
            };
            Some(run_automation_native(&[
                "commit-msg-check".to_string(),
                "--file".to_string(),
                file,
            ]))
        }
        "prepare-commit-msg" => {
            let Some(file) = args.get(1).cloned() else {
                return Some(0);
            };
            let mut hook_args = vec!["prepare-commit-msg".to_string(), "--file".to_string(), file];
            if let Some(source) = args.get(2) {
                hook_args.push("--source".to_string());
                hook_args.push(source.clone());
            }
            Some(run_automation_native(&hook_args))
        }
        "pre-branch-create" => {
            let Some(branch) = args.get(1).cloned() else {
                return Some(1);
            };
            Some(run_automation_native(&[
                "pre-branch-create-check".to_string(),
                "--branch".to_string(),
                branch,
            ]))
        }
        "branch-creation-check" => {
            let mut hook_args = vec!["branch-creation-check".to_string()];
            hook_args.extend(args.iter().skip(1).cloned());
            Some(run_git_native(&hook_args))
        }
        _ => None,
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
