use crate::pr::commands::pr_update_body_options::PrUpdateBodyOptions;
use crate::pr::gh_cli::gh_status;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_update_body(opts: PrUpdateBodyOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    gh_status(
        "pr",
        &[
            "edit",
            &opts.pr_number,
            "-R",
            &repo_name,
            "--body",
            &opts.body,
        ],
    )
}
