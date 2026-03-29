//! tools/versioning_automation/src/open_pr_issue_refs.rs
use std::collections::BTreeSet;

use crate::{gh_cli, pr::extract_effective_issue_ref_records};

pub(crate) fn load_open_pr_numbers_referencing_issue(
    issue_number: &str,
    repo_name: &str,
) -> Result<Vec<String>, String> {
    let issue_key = format!("#{issue_number}");
    let pr_rows = gh_cli::output_trim(&[
        "api",
        &format!("repos/{repo_name}/pulls?state=open&per_page=100"),
        "--paginate",
        "--jq",
        ".[]. | [.number, (.body // \"\")] | @tsv",
    ])?;

    let mut matched = BTreeSet::new();
    for line in pr_rows.lines() {
        let mut parts = line.splitn(2, '\t');
        let Some(pr_number) = parts.next() else {
            continue;
        };
        let body = parts.next().unwrap_or_default();
        if pr_body_references_issue(body, &issue_key) {
            matched.insert(pr_number.to_string());
        }
    }

    Ok(matched.into_iter().collect())
}

pub(crate) fn pr_body_references_issue(body: &str, issue_key: &str) -> bool {
    for record in extract_effective_issue_ref_records(body) {
        if record.first == "Closes" && record.second == issue_key {
            return true;
        }
    }
    false
}
