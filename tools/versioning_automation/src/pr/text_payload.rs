use std::process::Command;

use crate::pr::commands::pr_text_payload_options::PrTextPayloadOptions;

pub(crate) fn run_text_payload(opts: PrTextPayloadOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    let title = gh_output(
        "pr",
        &[
            "view",
            &opts.pr_number,
            "-R",
            &repo_name,
            "--json",
            "title",
            "--jq",
            ".title // \"\"",
        ],
    )
    .unwrap_or_default();
    let body = gh_output(
        "pr",
        &[
            "view",
            &opts.pr_number,
            "-R",
            &repo_name,
            "--json",
            "body",
            "--jq",
            ".body // \"\"",
        ],
    )
    .unwrap_or_default();
    let commits = gh_output(
        "api",
        &[
            &format!("repos/{repo_name}/pulls/{}/commits", opts.pr_number),
            "--paginate",
            "--jq",
            ".[].commit.message",
        ],
    )
    .unwrap_or_default();

    print!("{title}\n{body}\n{commits}");
    0
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

fn gh_output(cmd: &str, args: &[&str]) -> Result<String, String> {
    let mut command = Command::new("gh");
    command.arg(cmd);
    for arg in args {
        command.arg(arg);
    }
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(text.trim_end_matches('\n').to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
