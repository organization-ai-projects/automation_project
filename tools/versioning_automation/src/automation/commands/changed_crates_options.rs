use std::collections::BTreeSet;

use crate::automation::{
    ensure_git_repo,
    execute::{find_crate_dir_for_file, repo_root},
    git_changed_files, read_crate_name,
};

#[derive(Debug, Clone)]
pub(crate) struct ChangedCratesOptions {
    pub(crate) ref1: Option<String>,
    pub(crate) ref2: Option<String>,
    pub(crate) output_format: Option<String>,
}

impl ChangedCratesOptions {
    pub(crate) fn run_changed_crates(self) -> Result<(), String> {
        ensure_git_repo()?;
        let changed_files = git_changed_files(self.ref1.as_deref(), self.ref2.as_deref())?;
        if changed_files.is_empty() {
            println!("No changed files.");
            return Ok(());
        }

        let repo_root = repo_root()?;
        let mut crate_paths = BTreeSet::new();
        for file in changed_files {
            if let Some(path) = find_crate_dir_for_file(&repo_root, &file) {
                crate_paths.insert(path);
            }
        }

        if crate_paths.is_empty() {
            println!("No crates affected.");
            return Ok(());
        }

        let output_paths_only = self.output_format.as_deref() == Some("paths");
        if output_paths_only {
            for path in crate_paths {
                println!("{path}");
            }
            return Ok(());
        }

        println!("Changed crates:");
        for path in crate_paths {
            let crate_name = read_crate_name(&repo_root, &path).unwrap_or_else(|| path.clone());
            println!("  - {crate_name} ({path})");
        }
        Ok(())
    }
}
