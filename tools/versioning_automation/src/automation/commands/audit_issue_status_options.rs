//! tools/versioning_automation/src/automation/commands/audit_issue_status_options.rs
use std::fs;

use common_json::Json;

use crate::{
    automation::{
        audit_issue_status::{extract_issue_refs_from_text, render_issue_audit_report},
        execute::{
            ensure_git_repo, object_string, object_u64, parse_json_array, run_git_output_preserve,
        },
    },
    gh_cli,
    parent_field::extract_parent_field,
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct AuditIssueStatusOptions {
    pub(crate) repo: Option<String>,
    pub(crate) base_ref: String,
    pub(crate) head_ref: String,
    pub(crate) limit: usize,
    pub(crate) output_file: Option<String>,
}

impl AuditIssueStatusOptions {
    pub(crate) fn run_audit_issue_status(self) -> Result<(), String> {
        ensure_git_repo()?;
        let repo = resolve_repo_name(self.repo).map_err(|e| e.to_string())?;
        let range = format!("{}..{}", self.base_ref, self.head_ref);

        let open_issues_json = gh_cli::output_trim(&[
            "issue",
            "list",
            "--state",
            "open",
            "--limit",
            &self.limit.to_string(),
            "--json",
            "number,title,url,body,labels,state",
            "-R",
            &repo,
        ])
        .map_err(|e| format!("Failed to run gh issue list: {e}"))?;
        let open_issues = parse_json_array(&open_issues_json, "open issues JSON")?;
        let total_open = open_issues.len();

        let commit_messages = run_git_output_preserve(&["log", &range, "--format=%B"])?;
        let (closing_refs, reopen_refs, part_refs) =
            extract_issue_refs_from_text(&commit_messages)?;

        let mut would_close_items = Vec::new();
        let mut would_reopen_items = Vec::new();
        let mut part_only_items = Vec::new();
        let mut unreferenced_items = Vec::new();
        let mut done_in_dev_items = Vec::new();

        for issue in open_issues {
            let Some(obj) = issue.as_object() else {
                continue;
            };
            let number = object_u64(obj, "number");
            if number == 0 {
                continue;
            }
            let issue_id = number.to_string();
            let title = object_string(obj, "title");
            let url = object_string(obj, "url");
            let body = object_string(obj, "body");
            let parent = extract_parent_field(&body).unwrap_or_else(|| "(none)".to_string());

            let labels_csv = obj
                .get("labels")
                .and_then(Json::as_array)
                .map(|labels| {
                    labels
                        .iter()
                        .filter_map(|label| label.as_object())
                        .map(|label_obj| object_string(label_obj, "name").to_lowercase())
                        .collect::<Vec<_>>()
                        .join(",")
                })
                .unwrap_or_default();

            let line = format!("- [#{issue_id}]({url}) {title} (parent: {parent})");
            if labels_csv.contains("done-in-dev") {
                done_in_dev_items.push(line);
            } else if closing_refs.contains(&issue_id) {
                would_close_items.push(line);
            } else if reopen_refs.contains(&issue_id) {
                would_reopen_items.push(line);
            } else if part_refs.contains(&issue_id) {
                part_only_items.push(line);
            } else {
                unreferenced_items.push(line);
            }
        }

        let report = render_issue_audit_report(
            &repo,
            &range,
            total_open,
            (
                &done_in_dev_items,
                &would_close_items,
                &would_reopen_items,
                &part_only_items,
                &unreferenced_items,
            ),
        );

        if let Some(output_file) = self.output_file {
            fs::write(&output_file, &report)
                .map_err(|e| format!("Failed to write report to '{}': {e}", output_file))?;
            println!("Generated file: {output_file}");
        }
        print!("{report}");
        Ok(())
    }
}
