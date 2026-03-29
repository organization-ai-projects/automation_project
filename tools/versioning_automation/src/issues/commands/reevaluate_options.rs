//! tools/versioning_automation/src/issues/commands/reevaluate_options.rs
use crate::{
    issues::commands::NeutralizeOptions,
    open_pr_issue_refs::load_open_pr_numbers_referencing_issue, repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct ReevaluateOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}

impl ReevaluateOptions {
    pub(crate) fn run_reevaluate(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo) {
            Ok(repo) => repo,
            Err(msg) => {
                eprintln!("{msg}");
                return 3;
            }
        };

        let pr_numbers =
            load_open_pr_numbers_referencing_issue(&self.issue, &repo_name).unwrap_or_default();

        if pr_numbers.is_empty() {
            println!("No open PRs found referencing issue #{}.", self.issue);
            return 0;
        }

        let mut evaluated_count = 0usize;
        for pr_num in pr_numbers {
            println!(
                "Re-evaluating PR #{} (references issue #{})...",
                pr_num, self.issue
            );
            let status = NeutralizeOptions::run_neutralize(NeutralizeOptions {
                pr: pr_num.clone(),
                repo: Some(repo_name.clone()),
            });
            if status != 0 {
                return status;
            }
            evaluated_count += 1;
        }

        println!(
            "Re-evaluation complete. {} PR(s) evaluated.",
            evaluated_count
        );
        0
    }
}
