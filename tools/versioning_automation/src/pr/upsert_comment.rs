//! tools/versioning_automation/src/pr/upsert_comment.rs
use crate::issue_comment_upsert::upsert_issue_comment_by_marker;
use crate::pr::commands::PrUpsertCommentOptions;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_upsert_comment(opts: PrUpsertCommentOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    match upsert_issue_comment_by_marker(&repo_name, &opts.pr_number, &opts.marker, &opts.body) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}
