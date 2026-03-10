use std::process::Command;

use crate::pr::commands::pr_upsert_comment_options::PrUpsertCommentOptions;

pub(crate) fn run_upsert_comment(opts: PrUpsertCommentOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let list_path = format!("repos/{repo_name}/issues/{}/comments", opts.pr_number);
    let marker_query = opts.marker.replace('\\', "\\\\").replace('"', "\\\"");
    let jq_filter = format!(
        "map(select((.body // \"\") | contains(\"{marker_query}\"))) | sort_by(.updated_at) | last | .id // empty"
    );

    let comment_id =
        gh_output("api", &[&list_path, "--paginate", "--jq", &jq_filter]).unwrap_or_default();

    if comment_id.trim().is_empty() {
        gh_status("api", &[&list_path, "-f", &format!("body={}", opts.body)])
    } else {
        gh_status(
            "api",
            &[
                "-X",
                "PATCH",
                &format!("repos/{repo_name}/issues/comments/{}", comment_id.trim()),
                "-f",
                &format!("body={}", opts.body),
            ],
        )
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

fn gh_status(cmd: &str, args: &[&str]) -> i32 {
    let mut command = Command::new("gh");
    command.arg(cmd);
    for arg in args {
        command.arg(arg);
    }
    match command.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(err) => {
            eprintln!("Failed to execute gh {}: {err}", cmd);
            1
        }
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
