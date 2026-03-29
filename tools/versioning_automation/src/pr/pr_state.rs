//! tools/versioning_automation/src/pr/pr_state.rs
use crate::pr::commands::PrPrStateOptions;
use crate::pr_remote_snapshot::load_pr_remote_snapshot;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_pr_state(opts: PrPrStateOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    match load_pr_remote_snapshot(&opts.pr_number, &repo_name) {
        Ok(snapshot) => println!("{}", snapshot.state),
        Err(_) => println!(),
    }
    0
}
