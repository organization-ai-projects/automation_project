/// projects/products/varina/backend/src/git_github/mod.rs
pub mod git_parser;
pub mod policy_suggestions;

pub use git_parser::{parse_git_branch, parse_git_diff, parse_git_log_oneline, parse_git_show};
pub use policy_suggestions::{PolicySuggestion, suggest_policy_from_report};
