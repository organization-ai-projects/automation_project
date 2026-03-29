use crate::gh_cli::status_cmd;
use crate::pr::commands::PrIssueRefKindOptions;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_issue_ref_kind(opts: PrIssueRefKindOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let status = match status_cmd(
        "api",
        &[&format!("repos/{repo_name}/pulls/{}", opts.issue_number)],
    ) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Failed to execute gh api: {err}");
            1
        }
    };

    if status == 0 {
        println!("true");
        0
    } else {
        println!("false");
        0
    }
}
