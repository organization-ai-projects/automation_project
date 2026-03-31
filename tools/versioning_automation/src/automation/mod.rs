//! tools/versioning_automation/src/automation/mod.rs
mod audit_issue_status;
mod changed_crates;
mod check_dependencies;
mod clean_artifacts;
mod commands;
mod execute;
mod hook_checks;
mod install_hooks;
mod pre_add_review;
mod release_prepare;
mod render;
mod ui_build;

#[cfg(test)]
mod tests;

pub(crate) use audit_issue_status::{extract_issue_refs_from_text, render_issue_audit_report};
pub(crate) use changed_crates::{git_changed_files, read_crate_name};
pub(crate) use check_dependencies::run_check_dependencies;
pub(crate) use clean_artifacts::{
    remove_dir_if_exists, remove_files_by_suffixes, remove_named_dirs_under,
    remove_nested_cargo_locks,
};
pub(crate) use execute::{
    branch_exists_local, branch_exists_remote, current_branch, current_branch_name,
    ensure_git_repo, get_merged_branches, git_fetch_prune, has_upstream, is_protected_branch,
    last_deleted_branch_file, list_gone_branches, load_last_deleted_branch, require_clean_tree,
    require_non_protected_branch, resolve_branch_sha, run_git, run_git_output,
    run_git_output_allow_failure, run_git_passthrough, sanitize_description,
    save_last_deleted_branch, validate_branch_name, validate_branch_type, validate_commit_message,
};
pub(crate) use hook_checks::{
    print_affected_crates, restage_staged_files, run_pre_commit_markdown, run_pre_commit_rustfmt,
    run_pre_commit_shell_syntax, run_pre_push_docs_scripts_mode, run_pre_push_rust_mode,
    validate_pre_commit_assignment_policy, validate_pre_commit_branch_guard,
    validate_pre_push_commit_policies,
};
pub(crate) use render::print_usage;
