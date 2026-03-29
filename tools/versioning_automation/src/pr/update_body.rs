//! tools/versioning_automation/src/pr/update_body.rs
use crate::gh_cli::status_cmd;
use crate::pr::commands::PrUpdateBodyOptions;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_update_body(opts: PrUpdateBodyOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    match status_cmd(
        "pr",
        &[
            "edit",
            &opts.pr_number,
            "-R",
            &repo_name,
            "--body",
            &opts.body,
        ],
    ) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Failed to execute gh pr: {err}");
            1
        }
    }
}
