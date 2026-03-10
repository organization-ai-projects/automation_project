use std::process::Command;

use crate::pr::commands::pr_update_body_options::PrUpdateBodyOptions;

pub(crate) fn run_update_body(opts: PrUpdateBodyOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let mut edit = Command::new("gh");
    edit.arg("pr")
        .arg("edit")
        .arg(&opts.pr_number)
        .arg("-R")
        .arg(&repo_name)
        .arg("--body")
        .arg(&opts.body);

    match edit.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(err) => {
            eprintln!("Failed to execute gh pr edit: {err}");
            1
        }
    }
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
    let resolved = gh_output(
        "repo",
        &["view", "--json", "nameWithOwner", "--jq", ".nameWithOwner"],
    )
    .unwrap_or_default();
    if resolved.trim().is_empty() {
        Err("Error: unable to determine repository.".to_string())
    } else {
        Ok(resolved)
    }
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
