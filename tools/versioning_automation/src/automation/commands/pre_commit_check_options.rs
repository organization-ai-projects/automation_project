//! tools/versioning_automation/src/automation/commands/pre_commit_check_options.rs
use std::env;

use crate::automation::{
    execute, print_affected_crates, restage_staged_files, run_pre_commit_markdown,
    run_pre_commit_rustfmt, run_pre_commit_shell_syntax, validate_pre_commit_assignment_policy,
    validate_pre_commit_branch_guard,
};

#[derive(Debug)]
pub(crate) struct PreCommitCheckOptions;

impl PreCommitCheckOptions {
    pub(crate) fn run_pre_commit_check(opts: PreCommitCheckOptions) -> Result<(), String> {
        Self::run_pre_commit_check_with_skip(
            opts,
            env::var("SKIP_PRE_COMMIT").unwrap_or_default() == "1",
        )
    }

    pub(super) fn run_pre_commit_check_with_skip(
        opts: PreCommitCheckOptions,
        skip_pre_commit: bool,
    ) -> Result<(), String> {
        let _ = opts;
        if skip_pre_commit {
            println!("⚠️  Pre-commit checks skipped (SKIP_PRE_COMMIT=1)");
            return Ok(());
        }

        println!("📝 Running pre-commit checks...");
        println!();
        execute::ensure_git_repo()?;

        validate_pre_commit_branch_guard()?;
        let upstream = execute::resolve_upstream_or_default();
        let push_commits =
            execute::run_git_output_preserve(&["log", &format!("{upstream}..HEAD"), "--format=%B"])
                .unwrap_or_default();
        validate_pre_commit_assignment_policy(&push_commits)?;

        let staged_files = execute::list_staged_changed_files();
        if staged_files.is_empty() {
            println!("📝 No staged files detected; skipping file-based pre-commit checks");
            println!("✅ Pre-commit checks passed");
            println!();
            return Ok(());
        }
        let staged_changed_files = staged_files.join("\n");

        let crates =
            execute::collect_crates_from_changed_files(&staged_changed_files).unwrap_or_default();
        print_affected_crates(&crates);

        let markdown_files = execute::markdown_files_from(staged_files.as_slice());
        run_pre_commit_markdown(markdown_files.as_slice())?;
        run_pre_commit_shell_syntax(staged_files.as_slice())?;
        run_pre_commit_rustfmt(staged_files.as_slice())?;
        restage_staged_files()?;

        println!("✅ Pre-commit checks passed");
        println!();
        Ok(())
    }
}
