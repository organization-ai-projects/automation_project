//! tools/versioning_automation/src/automation/commands/ci_watch_pr_options.rs
use std::{
    thread,
    time::{Duration, Instant},
};

use common_json::Json;

use crate::{
    automation::execute::{object_string_or_default, parse_json_object, run_git_output},
    gh_cli,
};

#[derive(Debug)]
pub(crate) struct CiWatchPrOptions {
    pub(crate) pr_number: Option<String>,
    pub(crate) poll_interval: u64,
    pub(crate) max_wait: u64,
}

impl CiWatchPrOptions {
    pub(crate) fn run_ci_watch_pr(self) -> Result<(), String> {
        let pr_number = match self.pr_number {
            Some(value) => value,
            None => {
                let branch = run_git_output(&["branch", "--show-current"])?;
                if branch.trim().is_empty() {
                    return Err("No PR provided and unable to detect current branch.".to_string());
                }
                let value = gh_cli::output_trim(&[
                    "pr",
                    "list",
                    "--head",
                    branch.trim(),
                    "--json",
                    "number",
                    "--jq",
                    ".[0].number",
                ])
                .map_err(|e| {
                    format!(
                        "Failed to run gh pr list --head {} --json number --jq .[0].number: {e}",
                        branch.trim()
                    )
                })?;
                if value.trim().is_empty() || value.trim() == "null" {
                    return Err(format!("No PR found for branch '{}'.", branch.trim()));
                }
                value.trim().to_string()
            }
        };

        let start = Instant::now();
        loop {
            if start.elapsed().as_secs() > self.max_wait {
                return Err(format!("Timeout: CI not complete after {}s", self.max_wait));
            }

            let output = gh_cli::output_trim(&[
                "pr",
                "view",
                &pr_number,
                "--json",
                "state,mergeable,statusCheckRollup",
            ])
            .map_err(|e| {
                format!(
                    "Failed to run gh pr view {} --json state,mergeable,statusCheckRollup: {e}",
                    pr_number
                )
            })?;
            let parsed = parse_json_object(&output, "PR JSON")?;
            let checks = parsed
                .get("statusCheckRollup")
                .and_then(Json::as_array)
                .cloned()
                .unwrap_or_default();
            let total = checks.len();

            if total == 0 {
                thread::sleep(Duration::from_secs(self.poll_interval));
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

            thread::sleep(Duration::from_secs(self.poll_interval));
        }

        Ok(())
    }
}
