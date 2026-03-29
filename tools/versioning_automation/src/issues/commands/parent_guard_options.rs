//! tools/versioning_automation/src/issues/commands/parent_guard_options.rs
use crate::{
    issues::execute::{collect_parent_candidates, evaluate_parent_issue, split_repo_name},
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct ParentGuardOptions {
    pub(crate) issue: Option<String>,
    pub(crate) child: Option<String>,
    pub(crate) strict_guard: bool,
}

impl ParentGuardOptions {
    pub(crate) fn run_parent_guard(self) -> i32 {
        let repo_name = match resolve_repo_name(None) {
            Ok(repo) => repo,
            Err(msg) => {
                eprintln!("{msg}");
                return 3;
            }
        };
        let (repo_owner, repo_short_name) = split_repo_name(&repo_name);

        if let Some(issue_number) = self.issue.as_deref() {
            return evaluate_parent_issue(
                self.strict_guard,
                &repo_name,
                &repo_owner,
                &repo_short_name,
                issue_number,
            );
        }

        let Some(child_number) = self.child.as_deref() else {
            eprintln!("--issue or --child is required");
            return 2;
        };
        let candidates =
            collect_parent_candidates(&repo_name, &repo_owner, &repo_short_name, child_number);
        for parent_number in candidates {
            if parent_number == child_number {
                continue;
            }
            let status = evaluate_parent_issue(
                self.strict_guard,
                &repo_name,
                &repo_owner,
                &repo_short_name,
                &parent_number,
            );
            if status != 0 {
                return status;
            }
        }
        0
    }
}
