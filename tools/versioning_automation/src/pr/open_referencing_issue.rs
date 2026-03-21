//! tools/versioning_automation/src/pr/open_referencing_issue.rs
use crate::open_pr_issue_refs::load_open_pr_numbers_referencing_issue;
use crate::pr::commands::pr_open_referencing_issue_options::PrOpenReferencingIssueOptions;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_open_referencing_issue(opts: PrOpenReferencingIssueOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        return 0;
    };

    let matched =
        load_open_pr_numbers_referencing_issue(&opts.issue_number, &repo_name).unwrap_or_default();
    for pr_number in matched {
        println!("{pr_number}");
    }
    0
}
