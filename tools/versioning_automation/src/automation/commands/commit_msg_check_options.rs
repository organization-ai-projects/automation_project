//! tools/versioning_automation/src/automation/commands/commit_msg_check_options.rs
use std::{env, fs, path::PathBuf};

use crate::{
    automation::execute::{
        collect_format_categories, detect_required_scopes, extract_scopes_from_commit_subject,
        first_non_comment_subject_line, parse_subject_max_len, run_git_output_preserve,
    },
    issues,
    lazy_regex::COMMIT_MESSAGE_FORMAT_REGEX,
};

#[derive(Debug)]
pub(crate) struct CommitMsgCheckOptions {
    pub(crate) file: String,
}

impl CommitMsgCheckOptions {
    pub(crate) fn run_commit_msg_check(self) -> i32 {
        const RC_INVALID_FORMAT: i32 = 3;
        const RC_MIXED_CATEGORY: i32 = 6;
        const RC_SCOPE_MISSING: i32 = 7;
        const RC_SCOPE_MISMATCH: i32 = 8;

        if env::var("SKIP_COMMIT_VALIDATION").unwrap_or_default() == "1" {
            return 0;
        }

        let commit_msg_path = PathBuf::from(&self.file);
        if !commit_msg_path.is_file() {
            eprintln!("commit-msg-check: missing or invalid --file");
            return RC_INVALID_FORMAT;
        }

        let message = fs::read_to_string(&commit_msg_path)
            .map_err(|e| format!("Failed to read '{}': {e}", commit_msg_path.display()));
        let Ok(message) = message else {
            eprintln!("{}", message.unwrap_err());
            return RC_INVALID_FORMAT;
        };
        let subject = first_non_comment_subject_line(&message);
        let subject = subject.as_deref().unwrap_or_default();

        match parse_subject_max_len() {
            Ok(Some(max_len)) if subject.chars().count() > max_len => {
                eprintln!(
                    "Commit subject too long: {}/{} characters.",
                    subject.chars().count(),
                    max_len
                );
                return 9;
            }
            Ok(_) => {}
            Err(message) => {
                eprintln!("{message}");
                return RC_INVALID_FORMAT;
            }
        }

        let format_re = match COMMIT_MESSAGE_FORMAT_REGEX.as_ref() {
            Ok(re) => re,
            Err(_) => {
                eprintln!("Failed to compile commit message regex");
                return RC_INVALID_FORMAT;
            }
        };
        if !format_re.is_match(subject) {
            eprintln!("Invalid commit message format: '{subject}'");
            return RC_INVALID_FORMAT;
        }

        let footer_status = issues::run(&[
            "validate-footer".to_string(),
            "--file".to_string(),
            self.file.clone(),
        ]);
        if footer_status != 0 {
            return footer_status;
        }

        let staged_files_text = match run_git_output_preserve(&[
            "diff",
            "--cached",
            "--name-only",
            "--diff-filter=ACMRUD",
        ]) {
            Ok(value) => value,
            Err(message) => {
                eprintln!("{message}");
                return 1;
            }
        };
        let staged_files = staged_files_text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let format_categories = collect_format_categories(&staged_files);
        if format_categories.len() > 1 {
            eprintln!(
                "Mixed file format categories are not allowed in one commit: {}",
                format_categories.join(", ")
            );
            return RC_MIXED_CATEGORY;
        }

        let required_scopes = match detect_required_scopes(&staged_files) {
            Ok(value) => value,
            Err(message) => {
                eprintln!("{message}");
                return 1;
            }
        };
        if !required_scopes.is_empty() {
            let commit_scopes = extract_scopes_from_commit_subject(subject);
            if commit_scopes.is_empty() {
                eprintln!("Missing required scope in commit message.");
                return RC_SCOPE_MISSING;
            }
            let missing = required_scopes
                .into_iter()
                .filter(|required| !commit_scopes.iter().any(|scope| scope == required))
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                eprintln!(
                    "Commit scope does not match touched files. Missing: {}",
                    missing.join(", ")
                );
                return RC_SCOPE_MISMATCH;
            }
        }

        0
    }
}
