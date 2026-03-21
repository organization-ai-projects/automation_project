//! tools/versioning_automation/src/pr_run_snapshot.rs
use std::collections::{BTreeMap, BTreeSet};

use regex::Regex;

use crate::category_resolver::resolve_issue_outcome_category;
use crate::compare_snapshot::{CompareSnapshot, load_compare_snapshot};
use crate::pr;
use crate::pr::{
    DirectiveRecordType, IssueOutcomesSnapshot, build_issue_outcomes_snapshot,
    scan::scan_directives, text_indicates_breaking,
};

#[derive(Debug, Clone)]
pub(crate) struct PrRunSnapshot {
    pub(crate) compare: CompareSnapshot,
    pub(crate) validation_gate: String,
    pub(crate) duplicate_targets: BTreeMap<String, String>,
    pub(crate) issue_outcomes: IssueOutcomesSnapshot,
}

pub(crate) fn load_pr_run_snapshot(
    base_ref: &str,
    head_ref: &str,
) -> Result<PrRunSnapshot, String> {
    let compare = load_compare_snapshot(base_ref, head_ref)?;
    let validation_gate = build_validation_gate(&compare.commits);
    let duplicate_targets = collect_duplicate_targets(&compare.commits);
    let issue_outcomes =
        build_issue_outcomes_snapshot(&compare.commits, resolve_issue_outcome_category);

    Ok(PrRunSnapshot {
        compare,
        validation_gate,
        duplicate_targets,
        issue_outcomes,
    })
}

fn build_validation_gate(commits: &[pr::CommitInfo]) -> String {
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

fn collect_duplicate_targets(commits: &[pr::CommitInfo]) -> BTreeMap<String, String> {
    let text = commits
        .iter()
        .map(|commit| format!("{}\n{}", commit.subject, commit.body))
        .collect::<Vec<String>>()
        .join("\n\n");

    let mut targets = BTreeMap::new();
    for record in scan_directives(&text, true) {
        if record.record_type != DirectiveRecordType::Duplicate {
            continue;
        }
        if !record.first.is_empty() && !record.second.is_empty() {
            targets.insert(record.first, record.second);
        }
    }

    targets
}
