//! tools/versioning_automation/src/pr/issue_view.rs
use crate::issue_remote_snapshot::load_issue_remote_snapshot;
use crate::pr::commands::pr_issue_view_options::PrIssueViewOptions;
use crate::repo_name::resolve_repo_name_optional;
use common_json::to_string;

pub(crate) fn run_issue_view(opts: PrIssueViewOptions) -> i32 {
    let repo = resolve_repo_name_optional(opts.repo.as_deref());
    if let Ok(snapshot) = load_issue_remote_snapshot(&opts.issue_number, repo.as_deref())
        && let Ok(json) = to_string(&snapshot)
        && !json.is_empty()
    {
        println!("{json}");
    }
    0
}
