//! tools/versioning_automation/src/automation/execute.rs
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::thread;
use std::time::{Duration, Instant};

use common_json::Json;

use crate::automation::commands::{
    AutomationAction, CheckPriorityIssuesOptions, CiWatchPrOptions, LabelsSyncOptions,
    SyncMainDevCiOptions,
};
use crate::automation::parse::parse;
use crate::automation::render::print_usage;

pub(crate) fn run(args: &[String]) -> i32 {
    match parse(args) {
        Ok(action) => run_action(action),
        Err(message) => {
            eprintln!("{message}");
            2
        }
    }
}

fn run_action(action: AutomationAction) -> i32 {
    let result = match action {
        AutomationAction::Help => {
            print_usage();
            Ok(())
        }
        AutomationAction::CheckPriorityIssues(opts) => run_check_priority_issues(opts),
        AutomationAction::LabelsSync(opts) => run_labels_sync(opts),
        AutomationAction::CiWatchPr(opts) => run_ci_watch_pr(opts),
        AutomationAction::SyncMainDevCi(opts) => run_sync_main_dev_ci(opts),
    };

    match result {
        Ok(()) => 0,
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

fn run_check_priority_issues(opts: CheckPriorityIssuesOptions) -> Result<(), String> {
    let mut by_number: BTreeMap<u64, (String, String)> = BTreeMap::new();
    for label in ["high priority", "security"] {
        let mut args = vec![
            "issue",
            "list",
            "--state",
            "open",
            "--limit",
            "200",
            "--label",
            label,
            "--json",
            "number,title,url",
        ];
        if let Some(repo) = opts.repo.as_deref() {
            args.push("-R");
            args.push(repo);
        }
        let output = run_gh_output(&args)?;
        let issues = parse_json_array(&output, "issues JSON")?;
        for issue in issues {
            let Some(issue_object) = issue.as_object() else {
                continue;
            };
            let number = object_u64(issue_object, "number");
            if number == 0 {
                continue;
            }
            by_number.insert(
                number,
                (
                    object_string(issue_object, "title"),
                    object_string(issue_object, "url"),
                ),
            );
        }
    }

    if by_number.is_empty() {
        println!("No high priority or security issues found.");
        return Ok(());
    }

    println!("HIGH PRIORITY & SECURITY ISSUES");
    println!();
    for (idx, (number, (title, url))) in by_number.iter().enumerate() {
        println!("[{}] Issue #{}", idx + 1, number);
        println!("    Title: {}", title);
        println!("    URL:   {}", url);
        println!();
    }
    println!("Total priority issues: {}", by_number.len());

    Ok(())
}

fn run_labels_sync(opts: LabelsSyncOptions) -> Result<(), String> {
    let content = fs::read_to_string(&opts.labels_file).map_err(|e| {
        format!(
            "Labels file not found or unreadable '{}': {e}",
            opts.labels_file
        )
    })?;
    let labels = parse_labels(&content, &opts.labels_file)?;

    let existing = run_gh_output(&[
        "label", "list", "--limit", "1000", "--json", "name", "--jq", ".[].name",
    ])?;
    let mut existing_set: BTreeSet<String> = existing
        .lines()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .collect();

    for (name, color, description) in &labels {
        if name.trim().is_empty() {
            return Err("Label missing field 'name'".to_string());
        }
        if color.trim().is_empty() {
            return Err(format!("Label '{name}' missing field 'color'"));
        }

        if existing_set.contains(name) {
            run_gh_status(&[
                "label",
                "edit",
                name,
                "--color",
                color,
                "--description",
                description,
            ])?;
        } else {
            run_gh_status(&[
                "label",
                "create",
                name,
                "--color",
                color,
                "--description",
                description,
            ])?;
            existing_set.insert(name.clone());
        }
    }

    if opts.prune {
        let desired: BTreeSet<String> = labels
            .iter()
            .map(|(name, _, _)| name.clone())
            .filter(|name| !name.trim().is_empty())
            .collect();

        let repo_labels = run_gh_output(&[
            "label", "list", "--limit", "1000", "--json", "name", "--jq", ".[].name",
        ])?;
        let protected: BTreeSet<String> = [
            "bug",
            "documentation",
            "duplicate",
            "enhancement",
            "good first issue",
            "help wanted",
            "invalid",
            "question",
            "wontfix",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        for label in repo_labels
            .lines()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
        {
            if desired.contains(label) || protected.contains(label) {
                continue;
            }
            let _ = run_gh_status(&["label", "delete", label, "--yes"]);
        }
    }

    Ok(())
}

fn run_ci_watch_pr(opts: CiWatchPrOptions) -> Result<(), String> {
    let pr_number = match opts.pr_number {
        Some(value) => value,
        None => {
            let branch = run_git_output(&["branch", "--show-current"])?;
            if branch.trim().is_empty() {
                return Err("No PR provided and unable to detect current branch.".to_string());
            }
            let value = run_gh_output(&[
                "pr",
                "list",
                "--head",
                branch.trim(),
                "--json",
                "number",
                "--jq",
                ".[0].number",
            ])?;
            if value.trim().is_empty() || value.trim() == "null" {
                return Err(format!("No PR found for branch '{}'.", branch.trim()));
            }
            value.trim().to_string()
        }
    };

    let start = Instant::now();
    loop {
        if start.elapsed().as_secs() > opts.max_wait {
            return Err(format!("Timeout: CI not complete after {}s", opts.max_wait));
        }

        let output = run_gh_output(&[
            "pr",
            "view",
            &pr_number,
            "--json",
            "state,mergeable,statusCheckRollup",
        ])?;
        let parsed = parse_json_object(&output, "PR JSON")?;
        let checks = parsed
            .get("statusCheckRollup")
            .and_then(Json::as_array)
            .cloned()
            .unwrap_or_default();
        let total = checks.len();

        if total == 0 {
            thread::sleep(Duration::from_secs(opts.poll_interval));
            continue;
        }

        let pending = checks
            .iter()
            .filter(|check| {
                check
                    .as_object()
                    .and_then(|object| object.get("conclusion"))
                    .and_then(Json::as_str)
                    .is_none()
            })
            .count();
        let success = checks
            .iter()
            .filter(|check| {
                check
                    .as_object()
                    .and_then(|object| object.get("conclusion"))
                    .and_then(Json::as_str)
                    == Some("SUCCESS")
            })
            .count();
        let failure = checks
            .iter()
            .filter(|check| {
                check
                    .as_object()
                    .and_then(|object| object.get("conclusion"))
                    .and_then(Json::as_str)
                    == Some("FAILURE")
            })
            .count();
        let state = object_string_or_default(&parsed, "state", "UNKNOWN");
        let mergeable = object_string_or_default(&parsed, "mergeable", "UNKNOWN");

        println!(
            "[{} s] State: {} | Mergeable: {} | Checks: {}/{} passed, {} failed, {} pending",
            start.elapsed().as_secs(),
            state,
            mergeable,
            success,
            total,
            failure,
            pending
        );

        if failure > 0 {
            return Err(format!("CI failed for PR #{}.", pr_number));
        }

        if pending == 0 && success == total {
            println!("All checks passed for PR #{}.", pr_number);
            break;
        }

        thread::sleep(Duration::from_secs(opts.poll_interval));
    }

    Ok(())
}

fn run_sync_main_dev_ci(opts: SyncMainDevCiOptions) -> Result<(), String> {
    if std::env::var("CI").unwrap_or_default() != "true" {
        return Err("This command can only be executed in CI (CI=true).".to_string());
    }

    run_git(&["fetch", &opts.remote])?;

    let main_ref = format!("{}/{}", opts.remote, opts.main);
    let dev_ref = format!("{}/{}", opts.remote, opts.dev);

    let main_sha = run_git_output(&["rev-parse", &main_ref])?;
    let dev_sha = run_git_output(&["rev-parse", &dev_ref])?;
    if main_sha == dev_sha {
        println!("No sync needed - dev is already up to date with main");
        return Ok(());
    }

    if run_git(&["merge-base", "--is-ancestor", &main_ref, &dev_ref]).is_ok() {
        println!("No sync needed - dev already contains all commits from main");
        return Ok(());
    }

    if branch_exists_local(&opts.sync_branch) {
        let _ = run_git(&["branch", "-D", &opts.sync_branch]);
    }
    if branch_exists_remote(&opts.remote, &opts.sync_branch) {
        let _ = run_git(&["push", &opts.remote, "--delete", &opts.sync_branch]);
    }

    run_git(&["switch", "-C", &opts.sync_branch, &main_ref])?;
    run_git(&["push", "-f", &opts.remote, &opts.sync_branch])?;

    let pr_output = run_gh_output(&[
        "pr",
        "create",
        "--base",
        &opts.dev,
        "--head",
        &opts.sync_branch,
        "--title",
        "chore: sync main into dev",
        "--body",
        "Automated sync after merge into main.",
    ])?;

    let pr_url = pr_output.trim().to_string();
    if pr_url.is_empty() {
        return Err("Failed to create sync PR (empty response).".to_string());
    }

    let stable_timeout = std::env::var("STABLE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(120);
    let deadline = Instant::now() + Duration::from_secs(stable_timeout);

    let mergeable = loop {
        if Instant::now() >= deadline {
            return Err("PR did not stabilize in time.".to_string());
        }
        let value = run_gh_output(&[
            "pr",
            "view",
            &pr_url,
            "--json",
            "mergeable",
            "--jq",
            ".mergeable // \"UNKNOWN\"",
        ])?;
        if value != "UNKNOWN" {
            break value;
        }
        thread::sleep(Duration::from_secs(5));
    };

    if mergeable == "CONFLICTING" {
        return Err("PR has merge conflicts. Cannot enable auto-merge.".to_string());
    }
    if mergeable != "MERGEABLE" {
        return Err(format!("PR is not mergeable (status: {mergeable})."));
    }

    run_gh_status(&[
        "pr",
        "merge",
        &pr_url,
        "--auto",
        "--merge",
        "--delete-branch",
    ])?;
    Ok(())
}

fn run_git(args: &[&str]) -> Result<(), String> {
    crate::git_cli::status(args).map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

fn run_git_output(args: &[&str]) -> Result<String, String> {
    crate::git_cli::output_trim(args)
        .map_err(|e| format!("Failed to run git {}: {e}", args.join(" ")))
}

fn run_gh_status(args: &[&str]) -> Result<(), String> {
    crate::gh_cli::status(args).map_err(|e| format!("Failed to run gh {}: {e}", args.join(" ")))
}

fn run_gh_output(args: &[&str]) -> Result<String, String> {
    crate::gh_cli::output_trim(args)
        .map_err(|e| format!("Failed to run gh {}: {e}", args.join(" ")))
}

fn branch_exists_local(branch_name: &str) -> bool {
    crate::git_cli::status_success(&[
        "show-ref",
        "--verify",
        "--quiet",
        &format!("refs/heads/{branch_name}"),
    ])
}

fn branch_exists_remote(remote: &str, branch_name: &str) -> bool {
    crate::git_cli::status_success(&["ls-remote", "--exit-code", "--heads", remote, branch_name])
}

fn parse_json_array(payload: &str, context: &str) -> Result<Vec<Json>, String> {
    let parsed: Json = common_json::from_json_str(payload)
        .map_err(|e| format!("Failed to parse {context}: {e}"))?;
    parsed
        .as_array()
        .cloned()
        .ok_or_else(|| format!("Expected JSON array for {context}"))
}

fn parse_json_object(
    payload: &str,
    context: &str,
) -> Result<std::collections::HashMap<String, Json>, String> {
    let parsed: Json = common_json::from_json_str(payload)
        .map_err(|e| format!("Failed to parse {context}: {e}"))?;
    parsed
        .as_object()
        .cloned()
        .ok_or_else(|| format!("Expected JSON object for {context}"))
}

fn parse_labels(content: &str, source_name: &str) -> Result<Vec<(String, String, String)>, String> {
    let parsed = parse_json_array(content, &format!("labels JSON '{source_name}'"))?;
    let mut labels = Vec::with_capacity(parsed.len());
    for label in parsed {
        let Some(object) = label.as_object() else {
            return Err(format!(
                "Invalid label entry in '{source_name}': expected object"
            ));
        };
        labels.push((
            object_string(object, "name"),
            object_string(object, "color"),
            object_string(object, "description"),
        ));
    }
    Ok(labels)
}

fn object_u64(object: &std::collections::HashMap<String, Json>, key: &str) -> u64 {
    object.get(key).and_then(Json::as_u64).unwrap_or(0)
}

fn object_string(object: &std::collections::HashMap<String, Json>, key: &str) -> String {
    object
        .get(key)
        .and_then(Json::as_str)
        .unwrap_or_default()
        .to_string()
}

fn object_string_or_default(
    object: &std::collections::HashMap<String, Json>,
    key: &str,
    default: &str,
) -> String {
    let value = object_string(object, key);
    if value.trim().is_empty() {
        default.to_string()
    } else {
        value
    }
}
