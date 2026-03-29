//! tools/versioning_automation/src/pr/auto_add.rs
use std::collections::BTreeSet;

use crate::{gh_cli::output_trim_cmd, pr::extract_effective_issue_ref_records};

const AUTO_BLOCK_START: &str = "<!-- auto-closes:start -->";
const AUTO_BLOCK_END: &str = "<!-- auto-closes:end -->";

pub(crate) fn collect_refs_from_payload(payload: &str) -> (Vec<String>, Vec<String>) {
    let mut part_of_rows = BTreeSet::new();
    let mut closing_rows = BTreeSet::new();

    for record in extract_effective_issue_ref_records(payload) {
        if record.first == "Part of" {
            part_of_rows.insert(format!("Part of|{}", record.second));
        } else if record.first == "Closes" {
            closing_rows.insert(format!("Closes|{}", record.second));
        }
    }

    (
        part_of_rows.into_iter().collect(),
        closing_rows.into_iter().collect(),
    )
}

pub(crate) fn extract_issue_numbers(refs: &[String]) -> Vec<u32> {
    let mut issue_numbers = BTreeSet::new();
    for row in refs {
        let mut parts = row.split('|');
        let _action = parts.next();
        if let Some(issue_key) = parts.next()
            && let Some(number) = issue_key.strip_prefix('#')
            && let Ok(issue_number) = number.parse::<u32>()
        {
            issue_numbers.insert(issue_number);
        }
    }
    issue_numbers.into_iter().collect()
}

pub(crate) fn should_close_issue_for_author(
    issue_number: u32,
    repo_name: &str,
    pr_author: &str,
) -> bool {
    let assignees = output_trim_cmd(
        "issue",
        &[
            "view",
            &issue_number.to_string(),
            "-R",
            repo_name,
            "--json",
            "assignees",
            "--jq",
            ".assignees[].login",
        ],
    )
    .unwrap_or_default();

    let mut non_empty = assignees
        .lines()
        .map(str::trim)
        .filter(|line: &&str| !line.is_empty());
    if let Some(first) = non_empty.next() {
        if non_empty.next().is_some() {
            return false;
        }
        first == pr_author
    } else {
        false
    }
}

pub(crate) fn build_managed_block(issue_numbers: &BTreeSet<u32>) -> String {
    let mut out = String::new();
    out.push_str(AUTO_BLOCK_START);
    out.push('\n');
    out.push_str("### Auto-managed Issue Closures");
    out.push('\n');
    for n in issue_numbers {
        out.push_str("Closes #");
        out.push_str(&n.to_string());
        out.push('\n');
    }
    out.push_str(AUTO_BLOCK_END);
    out
}

pub(crate) fn strip_managed_block(body: &str) -> String {
    let mut out_lines = Vec::new();
    let mut in_block = false;
    for line in body.lines() {
        if line == AUTO_BLOCK_START {
            in_block = true;
            continue;
        }
        if line == AUTO_BLOCK_END {
            in_block = false;
            continue;
        }
        if !in_block {
            out_lines.push(line);
        }
    }
    out_lines.join("\n")
}

pub(crate) fn collapse_blank_runs(text: &str) -> String {
    let mut current = text.to_string();
    while current.contains("\n\n\n") {
        current = current.replace("\n\n\n", "\n\n");
    }
    current
}
