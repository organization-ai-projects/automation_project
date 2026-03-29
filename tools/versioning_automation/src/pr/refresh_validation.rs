//! tools/versioning_automation/src/pr/refresh_validation.rs
use serde::Deserialize;

use crate::gh_cli::{output_trim_end_newline_cmd, status_cmd};
use crate::pr::commands::PrRefreshValidationOptions;
use crate::pr_remote_snapshot::load_pr_remote_snapshot;
use crate::repo_name::resolve_repo_name;

#[derive(Debug, Deserialize)]
struct RefreshValidation {
    #[serde(default, rename = "statusCheckRollup")]
    status_check_rollup: Vec<std::collections::HashMap<String, String>>,
}

pub(crate) fn run_refresh_validation(opts: PrRefreshValidationOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let pr_snapshot = match load_pr_remote_snapshot(&opts.pr_number, &repo_name) {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{msg}");
            return 4;
        }
    };
    let status_snapshot = match fetch_status_check_rollup_snapshot(&opts.pr_number, &repo_name) {
        Ok(value) => value,
        Err(msg) => {
            eprintln!("{msg}");
            return 4;
        }
    };

    let ci_line =
        map_ci_status_with_symbol(compute_ci_status(&status_snapshot.status_check_rollup));
    let updated_body = apply_ci_refresh(&pr_snapshot.body, ci_line);
    if updated_body == pr_snapshot.body {
        println!("PR unchanged: #{}", opts.pr_number);
        return 0;
    }

    let status = match status_cmd(
        "pr",
        &[
            "edit",
            &opts.pr_number,
            "-R",
            &repo_name,
            "--body",
            &updated_body,
        ],
    ) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Failed to execute gh pr: {err}");
            1
        }
    };
    if status == 0 {
        println!("PR updated: #{}", opts.pr_number);
    }
    status
}

fn fetch_status_check_rollup_snapshot(
    pr_number: &str,
    repo_name: &str,
) -> Result<RefreshValidation, String> {
    let json = output_trim_end_newline_cmd(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "statusCheckRollup",
        ],
    )?;
    common_json::from_json_str::<RefreshValidation>(&json).map_err(|err| err.to_string())
}

fn compute_ci_status(items: &[std::collections::HashMap<String, String>]) -> &'static str {
    if items.is_empty() {
        return "UNKNOWN";
    }

    let mut has_pass = false;
    for item in items {
        let mut raw = item
            .get("conclusion")
            .map(String::as_str)
            .unwrap_or("")
            .trim();
        if raw.is_empty() {
            raw = item.get("state").map(String::as_str).unwrap_or("").trim();
        }
        if raw.is_empty() {
            raw = item.get("status").map(String::as_str).unwrap_or("").trim();
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
