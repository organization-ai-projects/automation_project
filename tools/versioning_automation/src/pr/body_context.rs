use std::process::Command;

use crate::pr::commands::pr_body_context_options::PrBodyContextOptions;
use crate::pr::contracts::github::issue_label::IssueLabel;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PrBodyContextSnapshot {
    #[serde(default)]
    title: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    labels: Vec<IssueLabel>,
}

pub(crate) fn run_body_context(opts: PrBodyContextOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        return 0;
    };

    let Ok(snapshot) = fetch_pr_snapshot(&opts.pr_number, &repo_name) else {
        return 0;
    };

    let labels_raw = snapshot
        .labels
        .iter()
        .map(|label| label.name.clone())
        .filter(|name| !name.trim().is_empty())
        .collect::<Vec<_>>()
        .join("||");
    println!("{}\x1f{}\x1f{}", snapshot.title, snapshot.body, labels_raw);
    0
}

fn fetch_pr_snapshot(pr_number: &str, repo_name: &str) -> Result<PrBodyContextSnapshot, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_number)
        .arg("-R")
        .arg(repo_name)
        .arg("--json")
        .arg("title,body,labels")
        .output()
        .map_err(|err| format!("Failed to execute gh pr view: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let json = String::from_utf8_lossy(&output.stdout).to_string();
    common_json::from_json_str::<PrBodyContextSnapshot>(&json).map_err(|err| err.to_string())
}

fn resolve_repo_name(explicit_repo: Option<String>) -> Result<String, String> {
    if let Some(repo) = explicit_repo.filter(|value| !value.trim().is_empty()) {
        return Ok(repo);
    }
    if let Ok(env_repo) = std::env::var("GH_REPO")
        && !env_repo.trim().is_empty()
    {
        return Ok(env_repo);
    }
    let output = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg("--json")
        .arg("nameWithOwner")
        .arg("-q")
        .arg(".nameWithOwner")
        .output()
        .map_err(|err| format!("Failed to execute gh repo view: {err}"))?;

    if !output.status.success() {
        return Err("Error: unable to determine repository.".to_string());
    }

    let repo = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if repo.is_empty() {
        Err("Error: unable to determine repository.".to_string())
    } else {
        Ok(repo)
    }
}
