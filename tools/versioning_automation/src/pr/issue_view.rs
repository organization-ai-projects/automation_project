//! tools/versioning_automation/src/pr/issue_view.rs
use crate::pr::commands::pr_issue_view_options::PrIssueViewOptions;
use crate::pr::gh_cli::gh_output_trim_end_newline;
use crate::repo_name::resolve_repo_name_optional;

pub(crate) fn run_issue_view(opts: PrIssueViewOptions) -> i32 {
    let resolved_repo = resolve_repo_name_optional(opts.repo.as_deref());
    let output = if let Some(repo_name) = resolved_repo.as_deref() {
        gh_output_trim_end_newline(
            "issue",
            &[
                "view",
                &opts.issue_number,
                "--json",
                "title,body,labels",
                "-R",
                repo_name,
            ],
        )
    } else {
        gh_output_trim_end_newline(
            "issue",
            &["view", &opts.issue_number, "--json", "title,body,labels"],
        )
    };

    if let Ok(json) = output
        && !json.is_empty()
    {
        println!("{json}");
    }
    0
}
