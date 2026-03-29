//! tools/versioning_automation/src/pr/commands/pr_auto_add_closes_options.rs
use std::collections::BTreeSet;

use crate::{
    gh_cli::status_cmd,
    pr::auto_add::{
        build_managed_block, collapse_blank_runs, collect_refs_from_payload, extract_issue_numbers,
        should_close_issue_for_author, strip_managed_block,
    },
    pr_remote_snapshot::PrRemoteSnapshot,
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrAutoAddClosesOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
}

impl PrAutoAddClosesOptions {
    pub(crate) fn run_auto_add_closes(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo) {
            Ok(repo) => repo,
            Err(msg) => {
                eprintln!("{msg}");
                return 3;
            }
        };

        let pr_snapshot =
            match PrRemoteSnapshot::load_pr_remote_snapshot(&self.pr_number, &repo_name) {
                Ok(snapshot) => snapshot,
                Err(_) => {
                    eprintln!("Error: unable to read PR #{}.", self.pr_number);
                    return 3;
                }
            };
        let pr_state = pr_snapshot.state;
        let pr_base = pr_snapshot.base_ref_name;
        let pr_title = pr_snapshot.title;
        let pr_body = pr_snapshot.body;
        let pr_author = pr_snapshot.author_login;

        if pr_state != "OPEN" {
            println!("PR #{} is not open; skipping.", self.pr_number);
            return 0;
        }
        if pr_base != "dev" {
            println!("PR #{} does not target dev; skipping.", self.pr_number);
            return 0;
        }
        if pr_author.is_empty() {
            println!(
                "PR #{}: author login unavailable; skipping.",
                self.pr_number
            );
            return 0;
        }

        let payload_all = format!("{pr_title}\n{pr_body}\n{}", pr_snapshot.commit_messages);

        let (part_of_refs, closing_refs) = collect_refs_from_payload(&payload_all);
        if part_of_refs.is_empty() {
            println!(
                "PR #{}: no Part of refs detected; nothing to enrich.",
                self.pr_number
            );
            return 0;
        }

        let mut already_closing = BTreeSet::new();
        for issue_number in extract_issue_numbers(&closing_refs) {
            already_closing.insert(issue_number);
        }

        let mut closes_to_add = BTreeSet::new();
        for issue_number in extract_issue_numbers(&part_of_refs) {
            if already_closing.contains(&issue_number) {
                continue;
            }
            if should_close_issue_for_author(issue_number, &repo_name, &pr_author) {
                closes_to_add.insert(issue_number);
            }
        }

        if closes_to_add.is_empty() {
            println!(
                "PR #{}: no qualifying single-assignee issue found; nothing to enrich.",
                self.pr_number
            );
            return 0;
        }

        let managed_block = build_managed_block(&closes_to_add);
        let body_without_block = collapse_blank_runs(&strip_managed_block(&pr_body));
        let new_body = if body_without_block.is_empty() {
            managed_block
        } else {
            format!("{body_without_block}\n\n{managed_block}")
        };

        if new_body == pr_body {
            println!("PR #{}: body already up-to-date.", self.pr_number);
            return 0;
        }

        let status = match status_cmd(
            "pr",
            &[
                "edit",
                &self.pr_number,
                "-R",
                &repo_name,
                "--body",
                &new_body,
            ],
        ) {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("Failed to execute gh pr: {err}");
                1
            }
        };
        if status == 0 {
            println!(
                "PR #{}: updated body with auto-managed Closes refs.",
                self.pr_number
            );
        }
        status
    }
}
