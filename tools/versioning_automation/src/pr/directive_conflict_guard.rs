//! tools/versioning_automation/src/pr/directive_conflict_guard.rs
use std::collections::BTreeSet;

use regex::Regex;

use crate::issue_comment_upsert::upsert_issue_comment_by_marker;

pub(crate) const BLOCK_START: &str = "<!-- directive-conflicts:start -->";
pub(crate) const BLOCK_END: &str = "<!-- directive-conflicts:end -->";

pub(crate) fn build_directive_payload(body: &str, commit_messages: &str) -> String {
    format!("{body}\n{commit_messages}")
}

pub(crate) fn detect_source_branch_count(commit_messages: &str) -> u32 {
    let merge_re =
        Regex::new(r"(?m)^Merge pull request #[0-9]+ from [^/]+/(.+)$").expect("valid regex");
    let mut branches = BTreeSet::new();
    for caps in merge_re.captures_iter(commit_messages) {
        if let Some(branch) = caps.get(1) {
            let value = branch.as_str().trim();
            if !value.is_empty() {
                branches.insert(value.to_string());
            }
        }
    }
    if branches.is_empty() {
        1
    } else {
        branches.len() as u32
    }
}

pub(crate) fn upsert_conflict_block_in_body(body: &str, block: Option<&str>) -> String {
    let block_re = Regex::new(&format!(
        r"\n?{}\n.*?\n{}\n?",
        regex::escape(BLOCK_START),
        regex::escape(BLOCK_END)
    ))
    .expect("valid regex");
    let without_block = block_re.replace(body, "").to_string();

    match block {
        Some(content) => format!("{without_block}\n\n{content}\n"),
        None => without_block,
    }
}

pub(crate) fn upsert_pr_comment(repo_name: &str, pr_number: &str, marker: &str, body: &str) -> i32 {
    match upsert_issue_comment_by_marker(repo_name, pr_number, marker, body) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}

fn issue_number(issue_key: &str) -> u32 {
    issue_key
        .trim_start_matches('#')
        .parse::<u32>()
        .unwrap_or(u32::MAX)
}
