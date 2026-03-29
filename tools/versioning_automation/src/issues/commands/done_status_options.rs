//! tools/versioning_automation/src/issues/commands/done_status_options.rs
use crate::{
    gh_cli::output_trim_or_empty,
    issue_remote_snapshot::IssueRemoteSnapshot,
    issues::{
        IssueSyncPlan,
        commands::{UpdateOptions, done_status_mode::DoneStatusMode},
        execute::{
            has_label_named, issue_remote_snapshot_or_default,
            load_effective_issue_action_numbers_for_pr,
        },
    },
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct DoneStatusOptions {
    pub(crate) mode: DoneStatusMode,
    pub(crate) pr: Option<String>,
    pub(crate) issue: Option<String>,
    pub(crate) label: String,
    pub(crate) repo: Option<String>,
}

impl DoneStatusOptions {
    pub(crate) fn run_done_status(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo.clone()) {
            Ok(repo) => repo,
            Err(message) => {
                eprintln!("{message}");
                return 3;
            }
        };
        let label_name = self.label;

        let label_exists = output_trim_or_empty(&[
            "label", "list", "-R", &repo_name, "--limit", "1000", "--json", "name", "--jq",
            ".[].name",
        ])
        .lines()
        .any(|value| value.trim() == label_name);

        match self.mode {
            DoneStatusMode::OnDevMerge => {
                let Some(pr_number) = self.pr else {
                    eprintln!("done-status --on-dev-merge requires: --pr");
                    return 2;
                };

                let pr_state = output_trim_or_empty(&[
                    "pr",
                    "view",
                    &pr_number,
                    "-R",
                    &repo_name,
                    "--json",
                    "state",
                    "--jq",
                    ".state // \"\"",
                ]);
                if pr_state != "MERGED" {
                    println!("PR #{} is not merged; nothing to do.", pr_number);
                    return 0;
                }

                let (closing_issue_numbers, _) =
                    match load_effective_issue_action_numbers_for_pr(&pr_number, &repo_name) {
                        Ok(value) => value,
                        Err(code) => return code,
                    };
                if closing_issue_numbers.is_empty() {
                    println!("No closing issue refs found for PR #{}.", pr_number);
                    return 0;
                }

                if !label_exists {
                    println!(
                        "Warning: label '{}' does not exist in {}; skipping done-in-dev labeling.",
                        label_name, repo_name
                    );
                    return 0;
                }

                for issue_number in closing_issue_numbers {
                    let snapshot = issue_remote_snapshot_or_default(&repo_name, &issue_number);
                    if snapshot.state.is_empty() {
                        println!("Issue #{}: unreadable; skipping.", issue_number);
                        continue;
                    }
                    let labels_raw = IssueRemoteSnapshot::issue_labels_raw(&snapshot);
                    let state = snapshot.state;
                    let sync_plan = IssueSyncPlan::plan_done_in_dev_sync(
                        &state,
                        has_label_named(&labels_raw, &label_name),
                    );
                    if !sync_plan.add_done_in_dev_label {
                        println!(
                            "Issue #{}: state={}; no done-in-dev label update needed.",
                            issue_number, state
                        );
                        continue;
                    }

                    if has_label_named(&labels_raw, &label_name) {
                        println!(
                            "Issue #{}: label '{}' already present.",
                            issue_number, label_name
                        );
                        continue;
                    }

                    let status = UpdateOptions::run_update(UpdateOptions {
                        issue: issue_number.clone(),
                        repo: Some(repo_name.clone()),
                        edit_args: vec![("--add-label".to_string(), label_name.clone())],
                    });
                    if status != 0 {
                        return status;
                    }
                }
                0
            }
            DoneStatusMode::OnIssueClosed => {
                let Some(issue_number) = self.issue else {
                    eprintln!("done-status --on-issue-closed requires: --issue");
                    return 2;
                };

                if !label_exists {
                    println!(
                        "Warning: label '{}' does not exist in {}; skipping.",
                        label_name, repo_name
                    );
                    return 0;
                }

                let has_label = output_trim_or_empty(&[
                    "issue",
                    "view",
                    &issue_number,
                    "-R",
                    &repo_name,
                    "--json",
                    "labels",
                    "--jq",
                    ".labels[].name",
                ])
                .lines()
                .any(|value| value.trim() == label_name);

                if has_label {
                    let status = UpdateOptions::run_update(UpdateOptions {
                        issue: issue_number.clone(),
                        repo: Some(repo_name.clone()),
                        edit_args: vec![("--remove-label".to_string(), label_name.clone())],
                    });
                    if status != 0 {
                        return status;
                    }
                } else {
                    println!(
                        "Issue #{}: label '{}' not present.",
                        issue_number, label_name
                    );
                }
                0
            }
        }
    }
}
