//! tools/versioning_automation/src/issues/commands/reopen_on_dev_options.rs
use std::env;

use crate::{
    gh_cli::output_trim_or_empty,
    issue_remote_snapshot::IssueRemoteSnapshot,
    issues::{
        IssueSyncPlan,
        commands::{IssueTarget, SyncProjectStatusOptions, UpdateOptions},
        execute::{
            has_label_named, issue_remote_snapshot_or_default,
            load_effective_issue_action_numbers_for_pr, pr_state_allows_reopen_sync,
        },
        run_sync_project_status,
    },
    pr_remote_snapshot::PrRemoteSnapshot,
    repo_name::resolve_repo_name,
};
#[derive(Debug, Clone)]
pub(crate) struct ReopenOnDevOptions {
    pub(crate) pr: String,
    pub(crate) label: String,
    pub(crate) repo: Option<String>,
}

impl ReopenOnDevOptions {
    pub(crate) fn run_reopen_on_dev(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo.clone()) {
            Ok(repo) => repo,
            Err(message) => {
                eprintln!("{message}");
                return 3;
            }
        };
        let label_name = self.label;
        let pr_number = self.pr;

        let pr_snapshot = match PrRemoteSnapshot::load_pr_remote_snapshot(&pr_number, &repo_name) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error: unable to read PR #{}.", pr_number);
                return 4;
            }
        };
        let pr_state = pr_snapshot.state;
        let pr_base = pr_snapshot.base_ref_name;
        if pr_base != "dev" {
            println!("PR #{} does not target dev; nothing to do.", pr_number);
            return 0;
        }
        if !pr_state_allows_reopen_sync(&pr_state) {
            println!(
                "PR #{} state={} is not eligible; nothing to do.",
                pr_number, pr_state
            );
            return 0;
        }

        let (_, reopen_issue_numbers) =
            match load_effective_issue_action_numbers_for_pr(&pr_number, &repo_name) {
                Ok(value) => value,
                Err(code) => return code,
            };
        if reopen_issue_numbers.is_empty() {
            println!("No reopen issue refs found for PR #{}.", pr_number);
            return 0;
        }

        let label_exists = output_trim_or_empty(&[
            "label", "list", "-R", &repo_name, "--limit", "1000", "--json", "name", "--jq",
            ".[].name",
        ])
        .lines()
        .any(|value| value.trim() == label_name);

        let reopen_status =
            env::var("PROJECT_STATUS_REOPEN_NAME").unwrap_or_else(|_| "Todo".to_string());

        for issue_number in reopen_issue_numbers {
            let snapshot = issue_remote_snapshot_or_default(&repo_name, &issue_number);
            if snapshot.state.is_empty() {
                println!("Issue #{}: unreadable; skipping reopen sync.", issue_number);
                continue;
            }
            let labels_raw = IssueRemoteSnapshot::issue_labels_raw(&snapshot);
            let state = snapshot.state;
            let sync_plan =
                IssueSyncPlan::plan_reopen_sync(&state, has_label_named(&labels_raw, &label_name));

            if sync_plan.reopen_issue {
                let status = IssueTarget::run_reopen(IssueTarget {
                    issue: issue_number.clone(),
                    repo: Some(repo_name.clone()),
                });
                if status != 0 {
                    return status;
                }
            } else {
                println!(
                    "Issue #{}: state={}; no reopen needed.",
                    issue_number, state
                );
            }

            if label_exists && sync_plan.remove_done_in_dev_label {
                let status = UpdateOptions::run_update(UpdateOptions {
                    issue: issue_number.clone(),
                    repo: Some(repo_name.clone()),
                    edit_args: vec![("--remove-label".to_string(), label_name.clone())],
                });
                if status != 0 {
                    return status;
                }
            }

            let status = run_sync_project_status(SyncProjectStatusOptions {
                repo: repo_name.clone(),
                issue: issue_number.clone(),
                status: reopen_status.clone(),
            });
            if status != 0 {
                return status;
            }
        }

        0
    }
}
