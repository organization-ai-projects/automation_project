use std::process::Command;

use serde::Deserialize;

use crate::pr::commands::pr_refresh_validation_options::PrRefreshValidationOptions;
use crate::repo_name::resolve_repo_name;

#[derive(Debug, Deserialize)]
struct PrValidationSnapshot {
    #[serde(default)]
    body: String,
    #[serde(default, rename = "statusCheckRollup")]
    status_check_rollup: Vec<PrValidationRollupItem>,
}

#[derive(Debug, Deserialize)]
struct PrValidationRollupItem {
    #[serde(default)]
    conclusion: String,
    #[serde(default)]
    state: String,
    #[serde(default)]
    status: String,
}

pub(crate) fn run_refresh_validation(opts: PrRefreshValidationOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let snapshot = match fetch_pr_validation_snapshot(&opts.pr_number, &repo_name) {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{msg}");
            return 4;
        }
    };

    let ci_line = map_ci_status_with_symbol(compute_ci_status(&snapshot.status_check_rollup));
    let updated_body = apply_ci_refresh(&snapshot.body, &ci_line);
    if updated_body == snapshot.body {
        println!("PR unchanged: #{}", opts.pr_number);
        return 0;
    }

    let mut edit = Command::new("gh");
    edit.arg("pr")
        .arg("edit")
        .arg(&opts.pr_number)
        .arg("-R")
        .arg(&repo_name)
        .arg("--body")
        .arg(&updated_body);

    match edit.status() {
        Ok(status) => {
            if status.success() {
                println!("PR updated: #{}", opts.pr_number);
                0
            } else {
                status.code().unwrap_or(1)
            }
        }
        Err(err) => {
            eprintln!("Failed to execute gh pr edit: {err}");
            1
        }
    }
}

fn fetch_pr_validation_snapshot(
    pr_number: &str,
    repo_name: &str,
) -> Result<PrValidationSnapshot, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_number)
        .arg("-R")
        .arg(repo_name)
        .arg("--json")
        .arg("body,statusCheckRollup")
        .output()
        .map_err(|err| format!("Failed to execute gh pr view: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let json = String::from_utf8_lossy(&output.stdout).to_string();
    common_json::from_json_str::<PrValidationSnapshot>(&json).map_err(|err| err.to_string())
}

fn compute_ci_status(items: &[PrValidationRollupItem]) -> &'static str {
    if items.is_empty() {
        return "UNKNOWN";
    }

    let mut has_pass = false;
    for item in items {
        let mut raw = item.conclusion.trim();
        if raw.is_empty() {
            raw = item.state.trim();
        }
        if raw.is_empty() {
            raw = item.status.trim();
        }
        let normalized = if raw.is_empty() {
            "UNKNOWN".to_string()
        } else {
            raw.to_ascii_uppercase()
        };

        match normalized.as_str() {
            "FAILURE" | "FAILED" | "CANCELLED" | "TIMED_OUT" | "ACTION_REQUIRED"
            | "STARTUP_FAILURE" => return "FAIL",
            "IN_PROGRESS" | "QUEUED" | "PENDING" | "WAITING" | "REQUESTED" => {
                return "RUNNING";
            }
            "SUCCESS" | "PASSED" => has_pass = true,
            "NEUTRAL" | "SKIPPED" | "COMPLETED" => {}
            _ => return "UNKNOWN",
        }
    }

    if has_pass { "PASS" } else { "UNKNOWN" }
}

fn map_ci_status_with_symbol(ci_status: &str) -> &'static str {
    match ci_status {
        "PASS" => "PASS ✅",
        "FAIL" => "FAIL ❌",
        "RUNNING" => "RUNNING ⏳",
        _ => "UNKNOWN ⚪",
    }
}

fn apply_ci_refresh(body: &str, ci_line: &str) -> String {
    let mut lines = body
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let had_trailing_newline = body.ends_with('\n');

    for line in &mut lines {
        if line.trim_start().starts_with("- CI:") {
            *line = format!("- CI: {ci_line}");
            let mut refreshed = lines.join("\n");
            if had_trailing_newline {
                refreshed.push('\n');
            }
            return refreshed;
        }
    }

    if let Some(index) = lines.iter().position(|line| {
        line.trim() == "### Validation Gate" || line.trim() == "### Validation Status"
    }) {
        let mut insert_at = index + 1;
        if insert_at < lines.len() && lines[insert_at].trim().is_empty() {
            insert_at += 1;
        }
        lines.insert(insert_at, format!("- CI: {ci_line}"));
        let mut refreshed = lines.join("\n");
        if had_trailing_newline {
            refreshed.push('\n');
        }
        return refreshed;
    }

    let mut base = body.trim_end().to_string();
    if !base.is_empty() {
        base.push_str("\n\n");
    }
    base.push_str("### Validation Gate\n\n");
    base.push_str(&format!("- CI: {ci_line}\n- No breaking change"));
    base
}

#[cfg(test)]
mod tests {
    use super::{
        PrValidationRollupItem, apply_ci_refresh, compute_ci_status, map_ci_status_with_symbol,
    };

    #[test]
    fn refresh_replaces_existing_ci_line() {
        let body = "### Validation Gate\n\n- CI: FAIL ❌\n- No breaking change\n";
        let refreshed = apply_ci_refresh(body, "PASS ✅");
        assert!(refreshed.contains("- CI: PASS ✅"));
        assert!(!refreshed.contains("- CI: FAIL ❌"));
    }

    #[test]
    fn refresh_inserts_validation_section_when_missing() {
        let body = "### Description\n\nHello";
        let refreshed = apply_ci_refresh(body, "RUNNING ⏳");
        assert!(refreshed.contains("### Validation Gate"));
        assert!(refreshed.contains("- CI: RUNNING ⏳"));
    }

    #[test]
    fn ci_status_detects_failure_first() {
        let items = vec![PrValidationRollupItem {
            conclusion: "failure".to_string(),
            state: String::new(),
            status: String::new(),
        }];
        assert_eq!(compute_ci_status(&items), "FAIL");
        assert_eq!(map_ci_status_with_symbol("FAIL"), "FAIL ❌");
    }
}
