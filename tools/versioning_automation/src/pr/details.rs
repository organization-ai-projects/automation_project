use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::pr::commands::pr_details_options::PrDetailsOptions;

#[derive(Debug, Deserialize)]
struct GhPrDetailsSnapshot {
    #[serde(default)]
    number: u64,
    #[serde(default)]
    url: String,
    #[serde(default)]
    state: String,
    #[serde(default, rename = "baseRefName")]
    base_ref_name: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    body: String,
    #[serde(default)]
    author: Option<GhPrAuthor>,
}

#[derive(Debug, Deserialize)]
struct GhPrAuthor {
    #[serde(default)]
    login: String,
}

#[derive(Debug, Serialize)]
struct PrDetailsOutput {
    number: u64,
    url: String,
    state: String,
    base_ref_name: String,
    author_login: String,
    title: String,
    body: String,
    commit_messages: String,
}

pub(crate) fn run_details(opts: PrDetailsOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    let pr_snapshot =
        fetch_pr_snapshot(&opts.pr_number, &repo_name).unwrap_or(GhPrDetailsSnapshot {
            number: 0,
            url: String::new(),
            state: String::new(),
            base_ref_name: String::new(),
            title: String::new(),
            body: String::new(),
            author: None,
        });
    let commit_messages = fetch_commit_messages(&opts.pr_number, &repo_name).unwrap_or_default();
    let author_login = pr_snapshot
        .author
        .as_ref()
        .map(|author| author.login.clone())
        .unwrap_or_default();

    let output = PrDetailsOutput {
        number: pr_snapshot.number,
        url: pr_snapshot.url,
        state: pr_snapshot.state,
        base_ref_name: pr_snapshot.base_ref_name,
        author_login,
        title: pr_snapshot.title,
        body: pr_snapshot.body,
        commit_messages,
    };

    match common_json::to_string_pretty(&output) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(err) => {
            eprintln!("failed to serialize pr details: {err}");
            1
        }
    }
}

fn fetch_pr_snapshot(pr_number: &str, repo_name: &str) -> Result<GhPrDetailsSnapshot, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_number)
        .arg("-R")
        .arg(repo_name)
        .arg("--json")
        .arg("number,url,state,baseRefName,title,body,author")
        .output()
        .map_err(|err| format!("Failed to execute gh pr view: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let json = String::from_utf8_lossy(&output.stdout).to_string();
    common_json::from_json_str::<GhPrDetailsSnapshot>(&json).map_err(|err| err.to_string())
}

fn fetch_commit_messages(pr_number: &str, repo_name: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .arg("api")
        .arg(format!("repos/{repo_name}/pulls/{pr_number}/commits"))
        .arg("--paginate")
        .arg("--jq")
        .arg(".[].commit.message")
        .output()
        .map_err(|err| format!("Failed to execute gh api pulls/commits: {err}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim_end_matches('\n')
        .to_string())
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
