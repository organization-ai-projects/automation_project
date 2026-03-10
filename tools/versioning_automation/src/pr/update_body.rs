use std::process::Command;

use crate::pr::commands::pr_update_body_options::PrUpdateBodyOptions;
use crate::repo_name::resolve_repo_name;

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
