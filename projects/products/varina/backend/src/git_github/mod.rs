//! projects/products/varina/backend/src/git_github/mod.rs
pub mod commands;
pub mod git_parser;
pub mod policy_suggestions;

pub(crate) use commands::{
    current_branch, ensure_git_repo, git_add_paths, git_commit, git_commit_dry_run,
    git_push_current_branch, git_reset, git_status_porcelain_z,
};
pub(crate) use policy_suggestions::suggest_policy_from_report;
