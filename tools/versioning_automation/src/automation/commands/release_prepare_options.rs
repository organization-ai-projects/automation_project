use crate::automation::{
    execute::{
        command_available, ensure_git_repo, repo_root, run_command_status, run_git, run_git_output,
    },
    release_prepare::{
        collect_files_named, require_clean_tree, update_changelog, update_version_in_cargo_file,
        validate_semver,
    },
};

#[derive(Debug, Clone)]
pub(crate) struct ReleasePrepareOptions {
    pub(crate) version: String,
    pub(crate) auto_changelog: bool,
}

impl ReleasePrepareOptions {
    pub(crate) fn run_release_prepare(self) -> Result<(), String> {
        ensure_git_repo()?;
        require_clean_tree()?;
        validate_semver(&self.version)?;

        let current_branch = run_git_output(&["branch", "--show-current"])?;
        if current_branch.trim() != "main" {
            println!(
                "Warning: current branch is '{}', not 'main'.",
                current_branch.trim()
            );
        }

        run_command_status("cargo", &["test", "--workspace"], false)?;

        if command_available("cargo-audit") {
            run_command_status("cargo", &["audit"], false)?;
        }

        let root = repo_root()?;
        let root_cargo = root.join("Cargo.toml");
        if root_cargo.is_file() {
            update_version_in_cargo_file(&root_cargo, &self.version)?;
        }

        let mut project_cargos = Vec::new();
        collect_files_named(&root.join("projects"), "Cargo.toml", &mut project_cargos)?;
        for cargo_toml in project_cargos {
            update_version_in_cargo_file(&cargo_toml, &self.version)?;
        }

        let changelog_path = root.join("CHANGELOG.md");
        if self.auto_changelog {
            update_changelog(&changelog_path, &self.version)?;
        } else {
            println!("Skipping automatic changelog generation.");
        }

        run_git(&["add", "-u"])?;
        let commit_message = format!(
            "chore: prepare release v{}\n\nRelease preparation for version {}.\n",
            self.version, self.version
        );
        run_git(&["commit", "-m", &commit_message])?;
        let tag_name = format!("v{}", self.version);
        run_git(&[
            "tag",
            "-a",
            &tag_name,
            "-m",
            &format!("Release {}", tag_name),
        ])?;
        println!("Release {} prepared.", tag_name);
        Ok(())
    }
}
