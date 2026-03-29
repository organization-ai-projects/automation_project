//! tools/versioning_automation/src/issues/commands/is_root_parent_options.rs
use crate::{
    issues::execute::{
        extract_subissue_refs_for_parent, issue_remote_snapshot_or_default, split_repo_name,
    },
    parent_field::extract_parent_field,
    repo_name::resolve_repo_name,
};

#[derive(Debug, Clone)]
pub(crate) struct IsRootParentOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}

impl IsRootParentOptions {
    pub(crate) fn run_is_root_parent(self) -> i32 {
        let repo_name = match resolve_repo_name(self.repo) {
            Ok(repo) => repo,
            Err(message) => {
                eprintln!("{message}");
                return 3;
            }
        };
        let body = issue_remote_snapshot_or_default(&repo_name, &self.issue).body;
        let parent_value = extract_parent_field(&body)
            .unwrap_or_else(|| "none".to_string())
            .to_lowercase();

        if parent_value == "epic" {
            println!("true");
            return 0;
        }
        if parent_value == "base" {
            println!("false");
            return 0;
        }
        if parent_value.starts_with('#') {
            println!("false");
            return 0;
        }

        let (owner, repo_short) = split_repo_name(&repo_name);
        let has_children =
            !extract_subissue_refs_for_parent(&owner, &repo_short, &self.issue).is_empty();
        println!("{}", if has_children { "true" } else { "false" });
        0
    }
}
