//! tools/versioning_automation/src/automation/parse.rs
use std::env;

use crate::automation::commands::{
    AuditSecurityOptions, AutomationAction, BuildAccountsUiOptions, BuildAndCheckUiBundlesOptions,
    BuildUiBundlesOptions, ChangedCratesOptions, CheckDependenciesOptions,
    CheckMergeConflictsOptions, CheckPriorityIssuesOptions, CiWatchPrOptions,
    CleanArtifactsOptions, LabelsSyncOptions, SyncMainDevCiOptions,
};

const DEFAULT_LABELS_FILE: &str = ".github/labels.json";

pub(crate) fn parse(args: &[String]) -> Result<AutomationAction, String> {
    if args.is_empty() {
        return Ok(AutomationAction::Help);
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" => Ok(AutomationAction::Help),
        "audit-security" => parse_audit_security(&args[1..]),
        "build-accounts-ui" => parse_build_accounts_ui(&args[1..]),
        "build-ui-bundles" => parse_build_ui_bundles(&args[1..]),
        "build-and-check-ui-bundles" => parse_build_and_check_ui_bundles(&args[1..]),
        "changed-crates" => parse_changed_crates(&args[1..]),
        "check-merge-conflicts" => parse_check_merge_conflicts(&args[1..]),
        "check-dependencies" => parse_check_dependencies(&args[1..]),
        "clean-artifacts" => parse_clean_artifacts(&args[1..]),
        "check-priority-issues" => parse_check_priority_issues(&args[1..]),
        "labels-sync" => parse_labels_sync(&args[1..]),
        "ci-watch-pr" => parse_ci_watch_pr(&args[1..]),
        "sync-main-dev-ci" => parse_sync_main_dev_ci(&args[1..]),
        unknown => Err(format!("Unknown automation subcommand: {unknown}")),
    }
}

fn parse_audit_security(args: &[String]) -> Result<AutomationAction, String> {
    if let Some(value) = args.first() {
        return Err(format!("Unexpected argument: {value}"));
    }
    Ok(AutomationAction::AuditSecurity(AuditSecurityOptions))
}

fn parse_build_accounts_ui(args: &[String]) -> Result<AutomationAction, String> {
    if let Some(value) = args.first() {
        return Err(format!("Unexpected argument: {value}"));
    }
    Ok(AutomationAction::BuildAccountsUi(BuildAccountsUiOptions))
}

fn parse_build_ui_bundles(args: &[String]) -> Result<AutomationAction, String> {
    if let Some(value) = args.first() {
        return Err(format!("Unexpected argument: {value}"));
    }
    Ok(AutomationAction::BuildUiBundles(BuildUiBundlesOptions))
}

fn parse_build_and_check_ui_bundles(args: &[String]) -> Result<AutomationAction, String> {
    if let Some(value) = args.first() {
        return Err(format!("Unexpected argument: {value}"));
    }
    Ok(AutomationAction::BuildAndCheckUiBundles(
        BuildAndCheckUiBundlesOptions,
    ))
}

fn parse_changed_crates(args: &[String]) -> Result<AutomationAction, String> {
    let mut ref1: Option<String> = None;
    let mut ref2: Option<String> = None;
    let mut output_format = env::var("OUTPUT_FORMAT").ok();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--output-format" => {
                i += 1;
                output_format = Some(required_value(args, i, "--output-format")?);
            }
            value if value.starts_with('-') => return Err(format!("Unexpected argument: {value}")),
            value => {
                if ref1.is_none() {
                    ref1 = Some(value.to_string());
                } else if ref2.is_none() {
                    ref2 = Some(value.to_string());
                } else {
                    return Err(format!("Unexpected argument: {value}"));
                }
            }
        }
        i += 1;
    }
    Ok(AutomationAction::ChangedCrates(ChangedCratesOptions {
        ref1,
        ref2,
        output_format,
    }))
}

fn parse_check_merge_conflicts(args: &[String]) -> Result<AutomationAction, String> {
    let mut remote = env::var("REMOTE").unwrap_or_else(|_| "origin".to_string());
    let mut base_branch = env::var("BASE_BRANCH").unwrap_or_else(|_| "dev".to_string());
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--remote" => {
                i += 1;
                remote = required_value(args, i, "--remote")?;
            }
            "--base-branch" | "--base" => {
                i += 1;
                base_branch = required_value(args, i, "--base-branch")?;
            }
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(AutomationAction::CheckMergeConflicts(
        CheckMergeConflictsOptions {
            remote,
            base_branch,
        },
    ))
}

fn parse_check_dependencies(args: &[String]) -> Result<AutomationAction, String> {
    let mut check_outdated = true;
    let mut check_unused = true;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--skip-outdated" => check_outdated = false,
            "--skip-unused" => check_unused = false,
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(AutomationAction::CheckDependencies(
        CheckDependenciesOptions {
            check_outdated,
            check_unused,
        },
    ))
}

fn parse_clean_artifacts(args: &[String]) -> Result<AutomationAction, String> {
    let mut include_node_modules = true;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--skip-node-modules" => include_node_modules = false,
            value => return Err(format!("Unexpected argument: {value}")),
        }
        i += 1;
    }
    Ok(AutomationAction::CleanArtifacts(CleanArtifactsOptions {
        include_node_modules,
    }))
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
