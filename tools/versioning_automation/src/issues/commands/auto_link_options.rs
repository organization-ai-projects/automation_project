//! tools/versioning_automation/src/issues/commands/auto_link_options.rs
use crate::{
    issue_remote_snapshot::IssueRemoteSnapshot,
    issues::{
        Validation,
        execute::{
            auto_link_extract_parent, auto_link_set_validation_error_state, is_issue_key,
            issue_remote_snapshot_or_default, run_auto_link_parent_link, run_auto_link_parent_none,
            split_repo_name,
        },
    },
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct AutoLinkOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}

impl AutoLinkOptions {
    pub(crate) fn run_auto_link(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo) {
            Ok(repo) => repo,
            Err(msg) => {
                eprintln!("{msg}");
                return 3;
            }
        };

        let marker = format!("<!-- parent-field-autolink:{} -->", self.issue);
        let label_required_missing = "issue-required-missing";
        let label_automation_failed = "automation-failed";
        let (repo_owner, repo_short_name) = split_repo_name(&repo_name);

        let issue_snapshot = issue_remote_snapshot_or_default(&repo_name, &self.issue);
        let issue_labels_raw = IssueRemoteSnapshot::issue_labels_raw(&issue_snapshot);
        let issue_title = issue_snapshot.title;
        let issue_body = issue_snapshot.body;
        let issue_state = issue_snapshot.state;
        if issue_state.is_empty() {
            eprintln!("Erreur: impossible de lire l'issue #{}.", self.issue);
            return 4;
        }

        let contract_errors =
            Validation::validate_content(&issue_title, &issue_body, &issue_labels_raw)
                .unwrap_or_default();
        if !contract_errors.is_empty() {
            let mut summary_lines = String::new();
            for entry in contract_errors {
                summary_lines.push_str("- ");
                summary_lines.push_str(&entry.message);
                summary_lines.push('\n');
            }
            let help = format!(
                "Detected problems:\n\n{summary_lines}\nExpected contract source: `.github/issue_required_fields.conf`."
            );
            let status = auto_link_set_validation_error_state(
                &repo_name,
                &self.issue,
                &marker,
                label_required_missing,
                label_automation_failed,
                "Issue body/title is non-compliant with required issue format.",
                &help,
            );
            return if status == 0 { 0 } else { status };
        }

        let parent_raw = match auto_link_extract_parent(&issue_body) {
            Some(value) if !value.is_empty() => value,
            _ => {
                let help = "Expected format:\n\n- `Parent: #<issue_number>` for child issues\n\n- `Parent: none` for independent issues\n\n- `Parent: base` for cascade root issues\n\n- `Parent: epic` for epic umbrella issues";
                let status = auto_link_set_validation_error_state(
                    &repo_name,
                    &self.issue,
                    &marker,
                    label_required_missing,
                    label_automation_failed,
                    "Missing required field `Parent:` in issue body.",
                    help,
                );
                return if status == 0 { 0 } else { status };
            }
        };

        let parent_raw_lc = parent_raw.to_lowercase();
        if parent_raw_lc == "none" || parent_raw_lc == "base" || parent_raw_lc == "epic" {
            return run_auto_link_parent_none(
                &repo_name,
                &repo_owner,
                &repo_short_name,
                &self.issue,
                &parent_raw_lc,
                &marker,
                label_required_missing,
                label_automation_failed,
            );
        }

        if !is_issue_key(&parent_raw) {
            let status = auto_link_set_validation_error_state(
                &repo_name,
                &self.issue,
                &marker,
                label_required_missing,
                label_automation_failed,
                &format!("Invalid `Parent:` value: `{parent_raw}`."),
                "Expected `Parent: #<issue_number>` or one of `Parent: none|base|epic`.",
            );
            return if status == 0 { 0 } else { status };
        }

        run_auto_link_parent_link(
            &repo_name,
            &repo_owner,
            &repo_short_name,
            &self.issue,
            parent_raw.trim_start_matches('#'),
            &marker,
            label_required_missing,
            label_automation_failed,
        )
    }
}
