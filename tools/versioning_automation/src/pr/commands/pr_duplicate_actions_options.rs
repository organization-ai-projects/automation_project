//! tools/versioning_automation/src/pr/commands/pr_duplicate_actions_options.rs
use crate::{gh_cli::status_cmd, pr::duplicate_actions::parse_duplicate_targets};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrDuplicateActionsOptions {
    pub(crate) text: String,
    pub(crate) mode: String,
    pub(crate) repo: String,
    pub(crate) assume_yes: bool,
}

impl PrDuplicateActionsOptions {
    pub(crate) fn run_duplicate_actions(self) -> i32 {
        let mode = self.mode.trim();
        if mode != "safe" && mode != "auto-close" {
            eprintln!("--mode must be safe or auto-close");
            return 2;
        }
        if self.repo.trim().is_empty() {
            eprintln!("--repo is required");
            return 2;
        }

        let duplicate_targets = parse_duplicate_targets(&self.text);
        if duplicate_targets.is_empty() {
            println!("Duplicate mode ({mode}): no duplicate declarations detected.");
            return 0;
        }

        let auto_close_allowed = mode != "auto-close" || self.assume_yes;
        if mode == "auto-close" && !self.assume_yes {
            eprintln!(
                "Warning: duplicate auto-close requested without --assume-yes; close action will be skipped."
            );
        }

        for (duplicate_issue_key, canonical_issue_key) in duplicate_targets {
            let duplicate_issue_number = duplicate_issue_key.trim_start_matches('#');
            let comment_body = if mode == "safe" {
                format!(
                    "Potential duplicate detected by PR generation workflow: {duplicate_issue_key} may duplicate {canonical_issue_key}. Please review manually."
                )
            } else {
                format!("Duplicate of {canonical_issue_key}")
            };

            let comment_status = match status_cmd(
                "api",
                &[
                    &format!(
                        "repos/{}/issues/{}/comments",
                        self.repo, duplicate_issue_number
                    ),
                    "-f",
                    &format!("body={comment_body}"),
                ],
            ) {
                Ok(()) => 0,
                Err(err) => {
                    eprintln!("Failed to execute gh api: {err}");
                    1
                }
            };
            if comment_status != 0 {
                return comment_status;
            }
            println!(
                "Duplicate mode ({mode}): commented on {duplicate_issue_key} (target {canonical_issue_key})."
            );

            if mode == "auto-close" && auto_close_allowed {
                let close_status = match status_cmd(
                    "api",
                    &[
                        &format!("repos/{}/issues/{}", self.repo, duplicate_issue_number),
                        "-X",
                        "PATCH",
                        "-f",
                        "state=closed",
                        "-f",
                        "state_reason=not_planned",
                    ],
                ) {
                    Ok(()) => 0,
                    Err(err) => {
                        eprintln!("Failed to execute gh api: {err}");
                        1
                    }
                };
                if close_status != 0 {
                    return close_status;
                }
                println!("Duplicate mode ({mode}): closed {duplicate_issue_key}.");
            } else if mode == "auto-close" {
                println!(
                    "Duplicate mode ({mode}): close skipped for {duplicate_issue_key} (missing --assume-yes)."
                );
            }
        }

        0
    }
}
