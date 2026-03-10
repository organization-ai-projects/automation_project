use std::process::Command;

use crate::pr::commands::pr_issue_ref_kind_options::PrIssueRefKindOptions;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_issue_ref_kind(opts: PrIssueRefKindOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let status = gh_status(
        "api",
        &[&format!("repos/{repo_name}/pulls/{}", opts.issue_number)],
    );

    if status == 0 {
        println!("true");
        0
    } else {
        println!("false");
        0
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
