//! tools/versioning_automation/src/pr/commit_info.rs
use std::collections::{self, BTreeMap, BTreeSet};

use common_json::Json;
use regex::Regex;

use crate::{
    category_resolver::classify_title,
    gh_cli::output_trim,
    pr::{
        DirectiveRecord, DirectiveRecordType, IssueOutcomesSnapshot,
        generate_description::render_change_footprint, text_indicates_breaking,
    },
    repo_name::resolve_repo_name_optional,
};

#[derive(Debug, Clone)]
pub(crate) struct CommitInfo {
    pub(crate) short_hash: String,
    pub(crate) subject: String,
    pub(crate) body: String,
}

impl CommitInfo {
    pub(crate) fn compare_api_commits(base_ref: &str, head_ref: &str) -> Result<Vec<Self>, String> {
        let Some(repo) = resolve_repo_name_optional(None) else {
            return Err("Error: unable to determine repository.".to_string());
        };

        let endpoint = format!("repos/{repo}/compare/{base_ref}...{head_ref}");
        let json = output_trim(&["api", &endpoint])?;
        Self::parse_compare_commits(&json)
    }

    pub(crate) fn parse_compare_commits(json: &str) -> Result<Vec<Self>, String> {
        let parsed: Json = common_json::from_json_str(json).map_err(|err| err.to_string())?;

        let mut commits = Vec::new();
        let commit_entries = parsed
            .as_object()
            .and_then(|object| object.get("commits"))
            .and_then(Json::as_array)
            .cloned()
            .unwrap_or_default();
        for entry in commit_entries {
            let Some(entry_object) = entry.as_object() else {
                continue;
            };
            let sha = entry_object
                .get("sha")
                .and_then(Json::as_str)
                .unwrap_or_default()
                .to_string();
            let message = entry_object
                .get("commit")
                .and_then(Json::as_object)
                .and_then(|commit_object| commit_object.get("message"))
                .and_then(Json::as_str)
                .unwrap_or_default()
                .trim()
                .to_string();
            if message.is_empty() {
                continue;
            }
            let mut lines = message.lines();
            let subject = lines.next().unwrap_or_default().trim().to_string();
            let body = lines.collect::<Vec<&str>>().join("\n").trim().to_string();
            commits.push(Self {
                short_hash: sha.chars().take(7).collect::<String>(),
                subject,
                body,
            });
        }

        Ok(commits)
    }

    pub(crate) fn build_validation_gate(commits: &[Self]) -> String {
        let ci_status = "UNKNOWN ⚪";

        let mut breaking_commit_hashes = BTreeSet::new();
        let mut breaking_scopes = BTreeSet::new();

        let scope_re =
            Regex::new(r"^[\s]*[a-z][a-z0-9_-]*\(([a-z0-9_./,-]+)\)!?:").expect("valid regex");

        for commit in commits {
            let combined = format!("{}\n{}", commit.subject, commit.body);
            if !text_indicates_breaking(&combined) {
                continue;
            }

            breaking_commit_hashes.insert(commit.short_hash.clone());

            if let Some(caps) = scope_re.captures(commit.subject.trim()) {
                let scope = caps.get(1).map(|m| m.as_str().trim()).unwrap_or_default();
                if !scope.is_empty() {
                    breaking_scopes.insert(scope.to_string());
                }
            }
        }

        let mut lines = vec![
            "### Validation Gate".to_string(),
            String::new(),
            format!("- CI: {ci_status}"),
        ];

        if breaking_commit_hashes.is_empty() {
            lines.push("- No breaking change".to_string());
        } else {
            lines.push("- Breaking change".to_string());
            lines.push("- Breaking scope:".to_string());

            if breaking_scopes.is_empty() {
                lines.push("  - crate(s): metadata-only (scope not inferable)".to_string());
            } else {
                let scopes = breaking_scopes
                    .iter()
                    .map(|v| format!("`{v}`"))
                    .collect::<Vec<String>>()
                    .join(", ");
                lines.push(format!("  - crate(s): {scopes}"));
            }

            let commits_value = breaking_commit_hashes
                .iter()
                .map(|v| format!("`{v}`"))
                .collect::<Vec<String>>()
                .join(", ");
            lines.push(format!("  - source commit(s): {commits_value}"));
        }

        lines.join("\n")
    }

    pub(crate) fn collect_duplicate_targets(commits: &[Self]) -> BTreeMap<String, String> {
        let text = commits
            .iter()
            .map(|commit| format!("{}\n{}", commit.subject, commit.body))
            .collect::<Vec<String>>()
            .join("\n\n");

        let mut targets = BTreeMap::new();
        for record in DirectiveRecord::scan_directives(&text, true) {
            if record.record_type != DirectiveRecordType::Duplicate {
                continue;
            }
            if !record.first.is_empty() && !record.second.is_empty() {
                targets.insert(record.first, record.second);
            }
        }

        targets
    }

    pub(crate) fn render_key_changes(commits: &[Self]) -> String {
        let mut groups: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();

        for commit in commits {
            if commit.subject.trim().is_empty() {
                continue;
            }
            let category = classify_title(&commit.subject);
            groups
                .entry(category)
                .or_default()
                .push(format!("- {}", commit.subject.trim()));
        }

        let ordered = ["Synchronization", "Features", "Bug Fixes", "Refactoring"];
        let mut parts = Vec::new();

        for name in ordered {
            let Some(lines) = groups.get(name) else {
                continue;
            };
            if lines.is_empty() {
                continue;
            }
            parts.push(format!("#### {name}"));
            parts.push(String::new());
            for line in lines {
                parts.push(line.clone());
            }
            parts.push(String::new());
        }

        if parts.is_empty() {
            "- No significant items detected.".to_string()
        } else {
            parts.join("\n").trim_end().to_string()
        }
    }

    pub(crate) fn build_full_body(
        base_ref: &str,
        head_ref: &str,
        commits: &[Self],
        range: &str,
        validation_gate: &str,
        issue_outcomes: &IssueOutcomesSnapshot,
    ) -> String {
        let mut out = String::new();

        out.push_str("### Description\n\n");
        out.push_str(&format!(
        "This pull request merges the `{head_ref}` branch into `{base_ref}` and summarizes merged pull requests and resolved issues.\n\n"
    ));

        out.push_str(validation_gate);
        out.push_str("\n\n");

        out.push_str("### Issue Outcomes\n\n");
        out.push_str(&IssueOutcomesSnapshot::render_issue_outcomes(
            issue_outcomes,
        ));
        out.push_str("\n\n");

        out.push_str("### Key Changes\n\n");
        out.push_str(&Self::render_key_changes(commits));
        out.push_str("\n\n");

        out.push_str("#### Change Footprint\n\n");
        out.push_str(&render_change_footprint(range));

        out.trim_end().to_string()
    }

    pub(crate) fn build_dynamic_pr_title(
        base_ref: &str,
        head_ref: &str,
        commits: &[Self],
    ) -> String {
        let mut has_sync = false;
        let mut has_features = false;
        let mut has_bugs = false;
        let mut has_refactors = false;

        for commit in commits {
            match classify_title(&commit.subject) {
                "Synchronization" => has_sync = true,
                "Features" => has_features = true,
                "Bug Fixes" => has_bugs = true,
                "Refactoring" => has_refactors = true,
                _ => {}
            }
        }

        let mut categories = Vec::new();
        if has_sync {
            categories.push("Synchronization");
        }
        if has_features {
            categories.push("Features");
        }
        if has_bugs {
            categories.push("Bug Fixes");
        }
        if has_refactors {
            categories.push("Refactoring");
        }

        let summary = if categories.is_empty() {
            "Changes".to_string()
        } else if categories.len() == 1 {
            categories[0].to_string()
        } else if categories.len() == 2 {
            format!("{} and {}", categories[0], categories[1])
        } else {
            let mut text = categories[0].to_string();
            for item in categories
                .iter()
                .skip(1)
                .take(categories.len().saturating_sub(2))
            {
                text.push_str(", ");
                text.push_str(item);
            }
            text.push_str(", and ");
            text.push_str(categories.last().copied().unwrap_or("Changes"));
            text
        };

        format!("Merge {head_ref} into {base_ref}: {summary}")
    }

    pub(crate) fn collect_default_categories(
        commits: &[Self],
    ) -> collections::BTreeMap<String, String> {
        let mut out = collections::BTreeMap::new();

        for commit in commits.iter().rev() {
            let category = classify_title(&commit.subject).to_string();
            let text = format!("{}\n{}", commit.subject, commit.body);
            for record in DirectiveRecord::scan_directives(&text, false) {
                if record.first != "Closes" && record.first != "Reopen" {
                    continue;
                }
                let issue = record.second.trim_start_matches('#');
                if issue.is_empty() {
                    continue;
                }
                out.insert(issue.to_string(), category.clone());
            }
        }

        out
    }
}
