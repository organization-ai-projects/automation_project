use std::path::Path;

use command_runner::CommandError;
use command_runner::run_cmd_ok;

use crate::commands::GIT_BIN;
use crate::commands::run_git_command;
use crate::repo_context::RepoContext;
use crate::repo_context::invalid;

// projects/libraries/git_lib/src/commit_context.rs
pub struct CommitContext<'a> {
    pub repo_context: RepoContext<'a>,
    pub subject: &'a str,
    pub body: &'a str,
}

impl<'a> CommitContext<'a> {
    /// Commits staged changes with a subject and optional body.
    /// Note: this does NOT stage files. Caller should stage first.
    pub fn git_commit(&mut self) -> Result<(), CommandError> {
        let subject = self.subject.trim();
        if subject.is_empty() {
            return Err(invalid("Sujet de commit vide"));
        }

        // Safety check: make sure something is staged (gives a clearer error)
        let staged = run_git_command(
            self.repo_context.repo_path,
            &["diff", "--cached", "--name-only"],
            self.repo_context.logs,
            false,
        )?;
        if staged.is_empty() {
            return Err(invalid("Aucun fichier mis en scène pour le commit"));
        }

        let commit_args = if self.body.trim().is_empty() {
            vec!["commit", "-m", subject]
        } else {
            vec!["commit", "-m", subject, "-m", self.body]
        };

        run_git_command(
            self.repo_context.repo_path,
            &commit_args,
            self.repo_context.logs,
            false,
        )?;
        Ok(())
    }

    /// Validates commit without applying changes.
    /// This requires staged files too (git checks it).
    pub fn git_commit_dry_run(&mut self) -> Result<(), CommandError> {
        let subject = self.subject.trim();
        if subject.is_empty() {
            return Err(invalid("Sujet de commit vide (dry-run)"));
        }

        if self.body.trim().is_empty() {
            run_cmd_ok(
                self.repo_context.repo_path,
                GIT_BIN,
                &["commit", "--dry-run", "-m", subject],
                self.repo_context.logs,
            )?;
        } else {
            run_cmd_ok(
                self.repo_context.repo_path,
                GIT_BIN,
                &["commit", "--dry-run", "-m", subject, "-m", self.body],
                self.repo_context.logs,
            )?;
        }

        Ok(())
    }
}

#[deprecated(note = "Utilisez CommitContext::git_commit_dry_run à la place.")]
pub fn git_commit_dry_run(
    repo_path: &Path,
    subject: &str,
    body: &str,
    logs: &mut Vec<String>,
) -> Result<(), CommandError> {
    let subject = subject.trim();
    if subject.is_empty() {
        return Err(invalid("Sujet de commit vide (dry-run)"));
    }

    if body.trim().is_empty() {
        run_cmd_ok(
            repo_path,
            GIT_BIN,
            &["commit", "--dry-run", "-m", subject],
            logs,
        )?;
    } else {
        run_cmd_ok(
            repo_path,
            GIT_BIN,
            &["commit", "--dry-run", "-m", subject, "-m", body],
            logs,
        )?;
    }

    Ok(())
}

/// Nouvelle fonction moderne pour valider un commit sans appliquer les changements.
pub fn git_commit_dry_run_with_context(
    repo_context: &mut RepoContext,
    subject: &str,
    body: &str,
) -> Result<(), CommandError> {
    let subject = subject.trim();
    if subject.is_empty() {
        return Err(invalid("Sujet de commit vide (dry-run)"));
    }

    if body.trim().is_empty() {
        run_cmd_ok(
            repo_context.repo_path,
            GIT_BIN,
            &["commit", "--dry-run", "-m", subject],
            repo_context.logs,
        )?;
    } else {
        run_cmd_ok(
            repo_context.repo_path,
            GIT_BIN,
            &["commit", "--dry-run", "-m", subject, "-m", body],
            repo_context.logs,
        )?;
    }

    Ok(())
}
