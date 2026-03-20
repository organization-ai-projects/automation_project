use crate::pr::commands::pr_issue_ref_kind_options::PrIssueRefKindOptions;
use crate::pr::gh_cli::gh_status;
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
