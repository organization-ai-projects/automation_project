//! tools/versioning_automation/src/automation/hook_checks/mod.rs
mod pre_commit;
mod pre_push;

#[cfg(test)]
mod tests;

pub(crate) use pre_commit::{
    print_affected_crates, restage_staged_files, run_pre_commit_markdown, run_pre_commit_rustfmt,
    run_pre_commit_shell_syntax, validate_pre_commit_assignment_policy,
    validate_pre_commit_branch_guard,
};
pub(crate) use pre_push::{
    build_quality_check_args, collect_changed_crates, run_pre_push_docs_scripts_mode,
    run_pre_push_rust_mode, validate_pre_push_commit_policies,
};
