//! tools/versioning_automation/src/automation/parse.rs
use std::env;

use crate::automation::commands::{
    AutomationAction, CheckPriorityIssuesOptions, CiWatchPrOptions, LabelsSyncOptions,
    SyncMainDevCiOptions,
};

const DEFAULT_LABELS_FILE: &str = ".github/labels.json";

pub(crate) fn parse(args: &[String]) -> Result<AutomationAction, String> {
    if args.is_empty() {
        return Ok(AutomationAction::Help);
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" => Ok(AutomationAction::Help),
        "check-priority-issues" => parse_check_priority_issues(&args[1..]),
        "labels-sync" => parse_labels_sync(&args[1..]),
        "ci-watch-pr" => parse_ci_watch_pr(&args[1..]),
        "sync-main-dev-ci" => parse_sync_main_dev_ci(&args[1..]),
        unknown => Err(format!("Unknown automation subcommand: {unknown}")),
    }
}

fn parse_check_priority_issues(args: &[String]) -> Result<AutomationAction, String> {
    let mut repo = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--repo" => {
                i += 1;
                repo = Some(required_value(args, i, "--repo")?);
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(AutomationAction::CheckPriorityIssues(
        CheckPriorityIssuesOptions { repo },
    ))
}

fn parse_labels_sync(args: &[String]) -> Result<AutomationAction, String> {
    let mut labels_file = DEFAULT_LABELS_FILE.to_string();
    let mut prune = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--prune" => prune = true,
            "--labels-file" => {
                i += 1;
                labels_file = required_value(args, i, "--labels-file")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(AutomationAction::LabelsSync(LabelsSyncOptions {
        labels_file,
        prune,
    }))
}

fn parse_ci_watch_pr(args: &[String]) -> Result<AutomationAction, String> {
    let mut pr_number = None;
    let mut poll_interval = std::env::var("POLL_INTERVAL")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(10);
    let mut max_wait = std::env::var("MAX_WAIT")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(3600);
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--pr" => {
                i += 1;
                pr_number = Some(required_value(args, i, "--pr")?);
            }
            "--poll-interval" => {
                i += 1;
                poll_interval = required_value(args, i, "--poll-interval")?
                    .parse::<u64>()
                    .map_err(|_| "--poll-interval must be a positive integer".to_string())?;
            }
            "--max-wait" => {
                i += 1;
                max_wait = required_value(args, i, "--max-wait")?
                    .parse::<u64>()
                    .map_err(|_| "--max-wait must be a positive integer".to_string())?;
            }
            value => {
                if pr_number.is_some() {
                    return Err(format!("Unexpected argument: {value}"));
                }
                pr_number = Some(value.to_string());
            }
        }
        i += 1;
    }
    Ok(AutomationAction::CiWatchPr(CiWatchPrOptions {
        pr_number,
        poll_interval,
        max_wait,
    }))
}

fn parse_sync_main_dev_ci(args: &[String]) -> Result<AutomationAction, String> {
    let mut remote = env::var("REMOTE").unwrap_or_else(|_| "origin".to_string());
    let mut main = env::var("MAIN").unwrap_or_else(|_| "main".to_string());
    let mut dev = env::var("DEV").unwrap_or_else(|_| "dev".to_string());
    let mut sync_branch = "sync/main-into-dev".to_string();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--main" => {
                i += 1;
                main = required_value(args, i, "--main")?;
            }
            "--dev" => {
                i += 1;
                dev = required_value(args, i, "--dev")?;
            }
            "--sync-branch" => {
                i += 1;
                sync_branch = required_value(args, i, "--sync-branch")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }

    Ok(AutomationAction::SyncMainDevCi(SyncMainDevCiOptions {
        remote,
        main,
        dev,
        sync_branch,
    }))
}

fn required_value(args: &[String], index: usize, option: &str) -> Result<String, String> {
    args.get(index)
        .cloned()
        .ok_or_else(|| format!("Option {option} requires a value."))
}
