//! tools/versioning_automation/src/pr/issue_outcomes_snapshot.rs
use std::collections::{self, BTreeSet};

use crate::category_resolver::classify_title;
use crate::pr::commit_info::CommitInfo;
use crate::pr::conflicts::build_conflict_report;
use crate::pr::domain::conflicts::resolved_conflict::ResolvedConflict;
use crate::pr::domain::conflicts::unresolved_conflict::UnresolvedConflict;
use crate::pr::generate_description::{
    render_directive_resolution_line, render_issue_outcome_entries,
    render_issue_outcome_groups_with_mode,
};
use crate::pr::{extract_effective_action_issue_numbers, scan_directives};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IssueOutcomesSnapshot {
    pub(crate) close_only: Vec<(String, String)>,
    pub(crate) reopen_only: Vec<(String, String)>,
    pub(crate) resolved_conflicts: Vec<(String, String, String, String)>,
    pub(crate) unresolved_conflicts: Vec<(String, String, String)>,
}

impl IssueOutcomesSnapshot {
    pub(crate) fn is_empty(&self) -> bool {
        self.close_only.is_empty()
            && self.reopen_only.is_empty()
            && self.resolved_conflicts.is_empty()
            && self.unresolved_conflicts.is_empty()
    }

    pub(crate) fn build_issue_outcomes_snapshot<F>(
        commits: &[CommitInfo],
        mut resolve_category: F,
    ) -> IssueOutcomesSnapshot
    where
        F: FnMut(&str, &str) -> String,
    {
        let text = commits
            .iter()
            .map(|commit| format!("{}\n{}", commit.subject, commit.body))
            .collect::<Vec<String>>()
            .join("\n\n");
        let (closes, reopens) = extract_effective_action_issue_numbers(&text);
        let conflict_report = build_conflict_report(&text, 1);
        let default_categories = collect_default_categories(commits);
        let resolved_conflict_keys = conflict_report
            .resolved
            .iter()
            .map(|entry| entry.issue.trim_start_matches('#').to_string())
            .collect::<BTreeSet<_>>();

        let close_only = closes
            .difference(&resolved_conflict_keys)
            .map(|issue| {
                let issue_key = issue_key(issue);
                let default_category = default_categories
                    .get(issue)
                    .map(String::as_str)
                    .unwrap_or("Unknown");
                (
                    issue_key.clone(),
                    resolve_category(&issue_key, default_category),
                )
            })
            .collect::<Vec<_>>();
        let reopen_only = reopens
            .difference(&resolved_conflict_keys)
            .map(|issue| {
                let issue_key = issue_key(issue);
                let default_category = default_categories
                    .get(issue)
                    .map(String::as_str)
                    .unwrap_or("Unknown");
                (
                    issue_key.clone(),
                    resolve_category(&issue_key, default_category),
                )
            })
            .collect::<Vec<_>>();
        let resolved_conflicts = conflict_report
            .resolved
            .iter()
            .map(|entry| resolved_entry(entry, &default_categories, &mut resolve_category))
            .collect::<Vec<_>>();
        let unresolved_conflicts = conflict_report
            .unresolved
            .iter()
            .map(|entry| unresolved_entry(entry, &default_categories, &mut resolve_category))
            .collect::<Vec<_>>();

        IssueOutcomesSnapshot {
            close_only,
            reopen_only,
            resolved_conflicts,
            unresolved_conflicts,
        }
    }

    pub(crate) fn render_issue_outcomes(snapshot: &IssueOutcomesSnapshot) -> String {
        if snapshot.is_empty() {
            return "- No issues processed in this PR.".to_string();
        }

        let close_rendered = render_issue_outcome_entries(&snapshot.close_only, "Closes");
        let reopen_rendered = render_issue_outcome_entries(&snapshot.reopen_only, "Reopen");
        let directive_resolution_records = snapshot
            .resolved_conflicts
            .iter()
            .map(|entry| {
                (
                    entry
                        .0
                        .trim_start_matches('#')
                        .parse::<u32>()
                        .unwrap_or(u32::MAX),
                    entry.1.clone(),
                    vec![render_directive_resolution_line(
                        &entry.0, &entry.2, &entry.3,
                    )],
                    0usize,
                )
            })
            .collect::<Vec<_>>();
        let unresolved_conflict_records = snapshot
            .unresolved_conflicts
            .iter()
            .map(|entry| {
                (
                    entry
                        .0
                        .trim_start_matches('#')
                        .parse::<u32>()
                        .unwrap_or(u32::MAX),
                    entry.1.clone(),
                    vec![entry.0.clone(), entry.2.clone()],
                    0usize,
                )
            })
            .collect::<Vec<_>>();
        let directive_rendered =
            render_issue_outcome_groups_with_mode(&directive_resolution_records, "directive")
                .trim()
                .to_string();
        let unresolved_rendered =
            render_issue_outcome_groups_with_mode(&unresolved_conflict_records, "conflict")
                .trim()
                .to_string();

        let mut out = String::new();
        out.push_str("#### Category 1: Issues Without Conflicts\n\n");
        out.push_str("##### Closes/Fixes\n\n");
        if close_rendered.trim().is_empty() {
            out.push_str(
                "- No resolved issues detected via GitHub references or PR body keywords.\n\n",
            );
        } else {
            out.push_str(close_rendered.trim());
            out.push_str("\n\n");
        }

        out.push_str("##### Reopened\n\n");
        if reopen_rendered.trim().is_empty() {
            out.push_str("- No reopened issues detected.\n\n");
        } else {
            out.push_str(reopen_rendered.trim());
            out.push_str("\n\n");
        }

        out.push_str("#### Category 2: Issues With Conflicts\n\n");
        out.push_str("##### Auto-resolved\n\n");
        if directive_rendered.trim().is_empty() {
            out.push_str("- No auto-resolved directive conflicts.\n\n");
        } else {
            out.push_str(directive_rendered.trim());
            out.push_str("\n\n");
        }

        out.push_str("##### Not resolved\n\n");
        if unresolved_rendered.trim().is_empty() {
            out.push_str("- No unresolved directive conflicts.");
        } else {
            out.push_str(unresolved_rendered.trim());
        }

        out
    }
}

fn resolved_entry<F>(
    entry: &ResolvedConflict,
    default_categories: &collections::BTreeMap<String, String>,
    resolve_category: &mut F,
) -> (String, String, String, String)
where
    F: FnMut(&str, &str) -> String,
{
    let issue = issue_key(&entry.issue);
    let default_category = default_categories
        .get(issue.trim_start_matches('#'))
        .map(String::as_str)
        .unwrap_or("Unknown");
    (
        issue.clone(),
        resolve_category(&issue, default_category),
        entry.decision.clone(),
        entry.origin.clone(),
    )
}

fn unresolved_entry<F>(
    entry: &UnresolvedConflict,
    default_categories: &collections::BTreeMap<String, String>,
    resolve_category: &mut F,
) -> (String, String, String)
where
    F: FnMut(&str, &str) -> String,
{
    let issue = issue_key(&entry.issue);
    let default_category = default_categories
        .get(issue.trim_start_matches('#'))
        .map(String::as_str)
        .unwrap_or("Unknown");
    (
        issue.clone(),
        resolve_category(&issue, default_category),
        entry.reason.clone(),
    )
}

fn collect_default_categories(commits: &[CommitInfo]) -> collections::BTreeMap<String, String> {
    let mut out = collections::BTreeMap::new();

    for commit in commits.iter().rev() {
        let category = classify_title(&commit.subject).to_string();
        let text = format!("{}\n{}", commit.subject, commit.body);
        for record in scan_directives(&text, false) {
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

fn issue_key(issue: &str) -> String {
    if issue.starts_with('#') {
        issue.to_string()
    } else {
        format!("#{issue}")
    }
}
