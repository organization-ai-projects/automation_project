//! tools/versioning_automation/src/compare_snapshot.rs
use common_json::Json;

use crate::gh_cli::output_trim;
use crate::pr::{CommitInfo, MainPrRefSnapshot};
use crate::repo_name::resolve_repo_name_optional;

#[derive(Debug, Clone)]
pub(crate) struct CompareSnapshot {
    pub(crate) base_ref: String,
    pub(crate) head_ref: String,
    pub(crate) commits: Vec<CommitInfo>,
}

pub(crate) fn fetch_pr_refs(pr_number: &str) -> Result<MainPrRefSnapshot, String> {
    let mut args = vec![
        "view".to_string(),
        pr_number.to_string(),
        "--json".to_string(),
        "baseRefName,headRefName".to_string(),
    ];
    if let Some(repo) = resolve_repo_name_optional(None) {
        args.push("-R".to_string());
        args.push(repo);
    }

    let borrowed = args.iter().map(String::as_str).collect::<Vec<&str>>();
    let mut full_args = vec!["pr"];
    full_args.extend(borrowed);
    let json = output_trim(&full_args)?;
    parse_pr_refs(&json)
}

pub(crate) fn load_compare_snapshot(
    base_ref: &str,
    head_ref: &str,
) -> Result<CompareSnapshot, String> {
    Ok(CompareSnapshot {
        base_ref: base_ref.to_string(),
        head_ref: head_ref.to_string(),
        commits: compare_api_commits(base_ref, head_ref)?,
    })
}

fn compare_api_commits(base_ref: &str, head_ref: &str) -> Result<Vec<CommitInfo>, String> {
    let Some(repo) = resolve_repo_name_optional(None) else {
        return Err("Error: unable to determine repository.".to_string());
    };

    let endpoint = format!("repos/{repo}/compare/{base_ref}...{head_ref}");
    let json = output_trim(&["api", &endpoint])?;
    parse_compare_commits(&json)
}

fn parse_pr_refs(json: &str) -> Result<MainPrRefSnapshot, String> {
    common_json::from_json_str::<MainPrRefSnapshot>(json).map_err(|err| err.to_string())
}

fn parse_compare_commits(json: &str) -> Result<Vec<CommitInfo>, String> {
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
        commits.push(CommitInfo {
            short_hash: sha.chars().take(7).collect::<String>(),
            subject,
            body,
        });
    }

    Ok(commits)
}
