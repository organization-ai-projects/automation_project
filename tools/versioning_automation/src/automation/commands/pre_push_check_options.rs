use std::env;

use crate::{
    automation::{
        execute, run_pre_push_docs_scripts_mode, run_pre_push_rust_mode,
        validate_pre_push_commit_policies,
    },
    repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct PrePushCheckOptions;

impl PrePushCheckOptions {
    pub(crate) fn run_pre_push_check(self) -> Result<(), String> {
        Self::run_pre_push_check_with_skip(
            self,
            env::var("SKIP_PRE_PUSH").unwrap_or_default() == "1",
        )
    }

    pub(crate) fn run_pre_push_check_with_skip(self, skip_pre_push: bool) -> Result<(), String> {
        let _ = self;
        if skip_pre_push {
            println!("Pre-push checks skipped (SKIP_PRE_PUSH=1)");
            return Ok(());
        }
        execute::ensure_git_repo()?;
        let upstream = execute::resolve_upstream_or_default();
        let commits =
            execute::run_git_output_preserve(&["log", &format!("{upstream}..HEAD"), "--format=%B"])
                .unwrap_or_default();
        let repo = repo_name::resolve_repo_name(None).ok();

        validate_pre_push_commit_policies(&commits, repo.as_deref())?;

        let changed_files = execute::compute_changed_files(&upstream)?;
        if changed_files.is_empty() {
            println!("Pre-push checks passed (no changed files)");
            return Ok(());
        }
        let markdown_files = execute::markdown_files_from(changed_files.as_slice());
        let docs_or_scripts_only = execute::is_docs_or_scripts_only_change(&changed_files);

        if docs_or_scripts_only {
            return run_pre_push_docs_scripts_mode(
                changed_files.as_slice(),
                markdown_files.as_slice(),
            );
        }

        run_pre_push_rust_mode(changed_files.as_slice(), markdown_files.as_slice())
    }
}
