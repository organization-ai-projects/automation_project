use std::path::Path;
use std::result::Result as StdResult;

use command_runner::{CommandError, CommandInfo};

use crate::commands::run_git_command;

pub const ADD_CHUNK: usize = 100;

pub fn invalid(reason: impl Into<String>) -> CommandError {
    CommandError::InvalidInput {
        info: CommandInfo {
            program: "git".to_string(),
            args: vec![],
        },
        reason: reason.into(),
    }
}

/// Definition of structures to group common fields
pub struct RepoContext<'a> {
    pub repo_path: &'a Path,
    pub logs: &'a mut Vec<String>,
}

impl<'a> RepoContext<'a> {
    /// Ensures the given path is a Git repository.
    pub fn ensure_git_repo(&mut self) -> StdResult<(), CommandError> {
        let out = run_git_command(
            self.repo_path,
            &["rev-parse", "--is-inside-work-tree"],
            self.logs,
            false,
        )?;
        if out != "true" {
            return Err(invalid("Refusal: not inside a Git repository."));
        }
        Ok(())
    }

    /// Retrieves the current branch and checks if HEAD is detached.
    pub fn current_branch(&mut self) -> StdResult<(String, bool), CommandError> {
        let out = run_git_command(
            self.repo_path,
            &["rev-parse", "--abbrev-ref", "HEAD"],
            self.logs,
            false,
        )?;
        let detached = out == "HEAD" || out.is_empty();
        Ok((out, detached))
    }

    /// Adds files to the Git index.
    pub fn git_add_paths(&mut self, paths: &[String]) -> StdResult<(), CommandError> {
        self.logs
            .push("[debug] git_add_paths: function entered".to_string());

        let cleaned: Vec<&str> = paths
            .iter()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect();

        if cleaned.is_empty() {
            return Ok(());
        }

        let relative_paths: Vec<String> = cleaned
            .iter()
            .filter(|p| self.repo_path.join(p).exists())
            .map(|p| {
                self.repo_path
                    .join(p)
                    .strip_prefix(self.repo_path)
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        for chunk in relative_paths.chunks(ADD_CHUNK) {
            let mut args = Vec::<&str>::with_capacity(2 + chunk.len());
            args.push("add");
            args.push("--");
            args.extend(chunk.iter().map(String::as_str));

            run_git_command(self.repo_path, &args, self.logs, false)?;
        }

        Ok(())
    }
}
