//! tools/versioning_automation/src/automation/commands/prepare_commit_msg_options.rs
use std::{env, fs, path::PathBuf};

use crate::automation::execute::{
    derive_description, detect_commit_type_from_context, detect_required_scopes,
    has_non_comment_content, run_git_output, run_git_output_preserve,
};
#[derive(Debug)]
pub(crate) struct PrepareCommitMsgOptions {
    pub(crate) file: String,
    pub(crate) source: Option<String>,
}

impl PrepareCommitMsgOptions {
    pub(crate) fn run_prepare_commit_msg(self) -> Result<(), String> {
        if env::var("SKIP_PREPARE_COMMIT_MSG").unwrap_or_default() == "1" {
            return Ok(());
        }

        let path = PathBuf::from(&self.file);
        if !path.is_file() {
            return Ok(());
        }
        let source = self.source.unwrap_or_default();
        if matches!(source.as_str(), "message" | "merge" | "squash" | "commit") {
            return Ok(());
        }

        let current = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
        if has_non_comment_content(&current) {
            return Ok(());
        }

        let branch = run_git_output(&["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default();
        if branch.trim().is_empty() || branch.trim() == "HEAD" {
            return Ok(());
        }

        let staged_files_text =
            run_git_output_preserve(&["diff", "--cached", "--name-only", "--diff-filter=ACMRU"])?;
        let staged_files = staged_files_text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        if staged_files.is_empty() {
            return Ok(());
        }

        let (commit_type, fallback_warning) = detect_commit_type_from_context(&staged_files);
        let scopes = detect_required_scopes(&staged_files)?;
        let scopes_csv = scopes.join(",");
        let description = derive_description(branch.trim(), &staged_files);
        let subject = if scopes_csv.is_empty() {
            format!("{commit_type}(workspace): {description}")
        } else {
            format!("{commit_type}({scopes_csv}): {description}")
        };

        let mut rendered = String::new();
        rendered.push_str(&subject);
        rendered.push_str("\n\n");
        rendered.push_str("# Auto-generated from branch and staged files.\n");
        if let Some(warning) = fallback_warning {
            rendered.push_str(warning);
            rendered.push('\n');
        }
        rendered.push_str("# Edit freely before saving this commit.\n");

        fs::write(&path, rendered)
            .map_err(|e| format!("Failed to write '{}': {e}", path.display()))?;
        Ok(())
    }
}
