use regex::Regex;
use std::collections::BTreeSet;
use std::process::Command;

use crate::pr::commands::pr_child_pr_refs_options::PrChildPrRefsOptions;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_child_pr_refs(opts: PrChildPrRefsOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        return 0;
    };

    let commit_headlines = fetch_commit_headlines(&opts.pr_number, &repo_name).unwrap_or_default();
    let pr_body = fetch_pr_body(&opts.pr_number, &repo_name).unwrap_or_default();
    let pr_comments = fetch_pr_comments(&opts.pr_number, &repo_name).unwrap_or_default();
    let timeline_refs = fetch_timeline_refs(&opts.pr_number, &repo_name).unwrap_or_default();

    let mut refs = BTreeSet::new();
    for issue_key in extract_refs_from_headlines(&commit_headlines) {
        refs.insert(issue_key);
    }
    for issue_key in extract_refs_from_text(&pr_body) {
        refs.insert(issue_key);
    }
    for issue_key in extract_refs_from_text(&pr_comments) {
        refs.insert(issue_key);
    }
    for issue_key in extract_timeline_refs(&timeline_refs) {
        refs.insert(issue_key);
    }

    let self_ref = format!("#{}", opts.pr_number);
    refs.remove(&self_ref);

    for issue_key in refs {
        println!("{issue_key}");
    }
    0
}

fn extract_refs_from_headlines(commit_headlines: &str) -> Vec<String> {
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

fn extract_refs_from_text(text: &str) -> Vec<String> {
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

fn extract_timeline_refs(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<u64>().ok())
        .map(|number| format!("#{number}"))
        .collect()
}

fn fetch_commit_headlines(pr_number: &str, repo_name: &str) -> Result<String, String> {
    gh_output(
        "api",
        &[
            &format!("repos/{repo_name}/pulls/{pr_number}/commits"),
            "--paginate",
            "--jq",
            ".[].commit.message | split(\"\\n\")[0]",
        ],
    )
}

fn fetch_pr_body(pr_number: &str, repo_name: &str) -> Result<String, String> {
    gh_output(
        "pr",
        &[
            "view",
            pr_number,
            "-R",
            repo_name,
            "--json",
            "body",
            "-q",
            ".body // \"\"",
        ],
    )
}

fn fetch_pr_comments(pr_number: &str, repo_name: &str) -> Result<String, String> {
    gh_output(
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

fn fetch_timeline_refs(pr_number: &str, repo_name: &str) -> Result<String, String> {
    gh_output(
        "api",
        &[
            &format!("repos/{repo_name}/issues/{pr_number}/timeline"),
            "--paginate",
            "--jq",
            ".[] | select(.event==\"cross-referenced\") | select(.source.issue.pull_request.url != null) | .source.issue.number",
        ],
    )
}

fn gh_output(cmd: &str, args: &[&str]) -> Result<String, String> {
    let mut command = Command::new("gh");
    command.arg(cmd);
    for arg in args {
        command.arg(arg);
    }
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(text)
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
