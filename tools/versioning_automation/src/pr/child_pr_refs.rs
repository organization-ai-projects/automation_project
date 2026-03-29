//! tools/versioning_automation/src/pr/child_pr_refs.rs
use regex::Regex;

use crate::gh_cli::output_trim_cmd;
pub(crate) fn extract_refs_from_headlines(commit_headlines: &str) -> Vec<String> {
    let merge_re = Regex::new(r"Merge pull request #([0-9]+)").expect("valid regex");
    let trailing_re = Regex::new(r"\(#([0-9]+)\)\s*$").expect("valid regex");
    let mut refs = Vec::new();

    for line in commit_headlines.lines() {
        if let Some(caps) = merge_re.captures(line) {
            refs.push(format!("#{}", &caps[1]));
        }
        if let Some(caps) = trailing_re.captures(line) {
            refs.push(format!("#{}", &caps[1]));
        }
    }

    refs
}

pub(crate) fn extract_refs_from_text(text: &str) -> Vec<String> {
    let pull_path_re = Regex::new(r"/pull/([0-9]+)").expect("valid regex");
    let pr_hash_re = Regex::new(r"(?i)\bPR\s*#([0-9]+)").expect("valid regex");
    let pull_request_hash_re = Regex::new(r"(?i)pull request #([0-9]+)").expect("valid regex");
    let mut refs = Vec::new();

    for caps in pull_path_re.captures_iter(text) {
        refs.push(format!("#{}", &caps[1]));
    }
    for caps in pr_hash_re.captures_iter(text) {
        refs.push(format!("#{}", &caps[1]));
    }
    for caps in pull_request_hash_re.captures_iter(text) {
        refs.push(format!("#{}", &caps[1]));
    }

    refs
}

pub(crate) fn extract_timeline_refs(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<u64>().ok())
        .map(|number| format!("#{number}"))
        .collect()
}

pub(crate) fn commit_headlines_from_messages(commit_messages: &str) -> String {
    commit_messages
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn fetch_pr_comments(pr_number: &str, repo_name: &str) -> Result<String, String> {
    output_trim_cmd(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "comments",
            "-q",
            ".comments[].body",
        ],
    )
}

pub(crate) fn fetch_timeline_refs(pr_number: &str, repo_name: &str) -> Result<String, String> {
    output_trim_cmd(
        "api",
        &[
            &format!("repos/{repo_name}/issues/{pr_number}/timeline"),
            "--paginate",
            "--jq",
            ".[] | select(.event==\"cross-referenced\") | select(.source.issue.pull_request.url != null) | .source.issue.number",
        ],
    )
}
