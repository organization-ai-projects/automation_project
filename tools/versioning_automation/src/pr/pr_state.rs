use std::process::Command;

use crate::pr::commands::pr_pr_state_options::PrPrStateOptions;
use crate::pr::contracts::github::pr_state_snapshot::PrStateSnapshot;

pub(crate) fn run_pr_state(opts: PrPrStateOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    match fetch_pr_state(&opts.pr_number, &repo_name) {
        Ok(state) => println!("{state}"),
        Err(_) => println!(),
    }
    0
}

fn fetch_pr_state(pr_number: &str, repo_name: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(pr_number)
        .arg("-R")
        .arg(repo_name)
        .arg("--json")
        .arg("state")
        .output()
        .map_err(|err| format!("Failed to execute gh pr view: {err}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            return Err("Error: unable to read PR state.".to_string());
        }
        return Err(stderr);
    }

    let json = String::from_utf8_lossy(&output.stdout).to_string();
    let snapshot = common_json::from_json_str::<PrStateSnapshot>(&json)
        .map_err(|err| format!("Error: invalid gh PR state payload: {err}"))?;
    Ok(snapshot.state)
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
