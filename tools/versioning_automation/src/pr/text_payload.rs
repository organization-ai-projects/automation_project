use std::process::Command;

use crate::pr::commands::pr_text_payload_options::PrTextPayloadOptions;
use crate::repo_name::resolve_repo_name;

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
