use std::process::Command;

use crate::pr::commands::pr_upsert_comment_options::PrUpsertCommentOptions;
use crate::repo_name::resolve_repo_name;

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
