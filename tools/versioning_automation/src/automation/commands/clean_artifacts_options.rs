//! tools/versioning_automation/src/automation/commands/clean_artifacts_options.rs
use crate::automation::{
    clean_artifacts::{
        remove_dir_if_exists, remove_files_by_suffixes, remove_named_dirs_under,
        remove_nested_cargo_locks,
    },
    execute::{ensure_git_repo, repo_root, run_command_status},
};

#[derive(Debug, Clone)]
pub(crate) struct CleanArtifactsOptions {
    pub(crate) include_node_modules: bool,
}

impl CleanArtifactsOptions {
    pub(crate) fn run_clean_artifacts(self) -> Result<(), String> {
        ensure_git_repo()?;
        let root = repo_root()?;

        remove_dir_if_exists(&root.join("target"))?;
        remove_named_dirs_under(&root.join("projects"), "ui_dist")?;
        if self.include_node_modules {
            remove_named_dirs_under(&root, "node_modules")?;
        }
        remove_nested_cargo_locks(&root.join("projects"), &root.join("Cargo.lock"))?;
        remove_files_by_suffixes(&root, &[".profraw", ".gcda", ".gcno", "~", ".bak", ".tmp"])?;

        run_command_status("cargo", &["clean"], false)?;
        println!("Build artifacts cleaned successfully.");
        Ok(())
    }
}
