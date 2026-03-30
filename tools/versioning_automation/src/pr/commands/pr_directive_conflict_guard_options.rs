use crate::{
    gh_cli::status_cmd,
    pr::{
        closure_marker::apply_marker,
        directive_conflict_guard::{
            build_directive_payload, detect_source_branch_count, upsert_conflict_block_in_body,
            upsert_pr_comment,
        },
        domain::conflicts::ConflictReport,
    },
    pr_remote_snapshot::PrRemoteSnapshot,
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrDirectiveConflictGuardOptions {
    pub(crate) pr_number: String,
    pub(crate) repo: Option<String>,
}

impl PrDirectiveConflictGuardOptions {
    pub(crate) fn run_directive_conflict_guard(self) -> i32 {
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
                    return 4;
                }
            };
        let original_body = pr_snapshot.body;
        let mut updated_body = original_body.clone();
        let commit_messages = pr_snapshot.commit_messages;
        let source_branch_count = detect_source_branch_count(&commit_messages);
        let directive_payload = build_directive_payload(&original_body, &commit_messages);

        let report = ConflictReport::build_conflict_report(&directive_payload, source_branch_count);
        let resolved_count = report.resolved.len();
        let unresolved_count = report.unresolved.len();

        for entry in &report.resolved {
            if entry.decision != "close" {
                continue;
            }
            match apply_marker(&updated_body, "reopen|reopens", &entry.issue) {
                Ok(next) => updated_body = next,
                Err(err) => {
                    eprintln!("{err}");
                    return 2;
                }
            }
        }

        let marker = format!("<!-- directive-conflict-guard:{} -->", self.pr_number);
        let conflict_block = ConflictReport::build_conflict_block(&report);
        updated_body = upsert_conflict_block_in_body(&updated_body, conflict_block.as_deref());

        if updated_body != original_body {
            let status = match status_cmd(
                "pr",
                &[
                    "edit",
                    &self.pr_number,
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
            if status != 0 {
                return status;
            }
        }

        if unresolved_count > 0 {
            let comment_body = format!(
                "{marker}\n### Directive Conflict Guard\n\n❌ Unresolved Closes/Reopen conflicts detected. Add explicit directive decisions in PR body."
            );
            let status = upsert_pr_comment(&repo_name, &self.pr_number, &marker, &comment_body);
            if status != 0 {
                return status;
            }
            eprintln!(
                "Unresolved directive conflicts detected for PR #{}.",
                self.pr_number
            );
            return 8;
        }

        if resolved_count > 0 {
            let comment_body = format!(
                "{marker}\n### Directive Conflict Guard\n\n✅ Directive conflicts resolved via explicit decisions."
            );
            let status = upsert_pr_comment(&repo_name, &self.pr_number, &marker, &comment_body);
            if status != 0 {
                return status;
            }
        }

        println!(
            "Directive conflict guard evaluated for PR #{}.",
            self.pr_number
        );
        0
    }
}
