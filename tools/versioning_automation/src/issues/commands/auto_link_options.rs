//! tools/versioning_automation/src/issues/commands/auto_link_options.rs
use crate::{
    issue_remote_snapshot::IssueRemoteSnapshot,
    issues::{
        Validation,
        execute::{
            auto_link_extract_parent, is_issue_key, issue_remote_snapshot_or_default,
            run_auto_link_parent_link, run_auto_link_parent_none, split_repo_name,
        },
    },
    repo_name::resolve_repo_name,
};

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AutoLinkError {
    MissingIssueField,
    UnableToReadIssue(String),
    RepoNotSpecified { issue: String, message: String },
    ValidationError(String),
    InvalidParentField(String),
}

impl fmt::Display for AutoLinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AutoLinkError::MissingIssueField => write!(f, "Missing issue field."),
            AutoLinkError::UnableToReadIssue(issue) => {
                write!(f, "Unable to read issue #{}.", issue)
            }
            AutoLinkError::RepoNotSpecified { issue, message } => {
                write!(
                    f,
                    "Repository not specified for issue #{}. Error: {}",
                    issue, message
                )
            }
            AutoLinkError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AutoLinkError::InvalidParentField(parent) => {
                write!(f, "Invalid `Parent:` value: `{}`.", parent)
            }
        }
    }
}

impl Error for AutoLinkError {}

#[derive(Debug, Clone)]
pub(crate) struct AutoLinkOptions {
    pub(crate) issue: String,
    pub(crate) repo: Option<String>,
}

impl AutoLinkOptions {
    pub(crate) fn run_auto_link(self) -> Result<(), AutoLinkError> {
        let repo_name =
            resolve_repo_name(self.repo).map_err(|msg| AutoLinkError::RepoNotSpecified {
                issue: self.issue.clone(),
                message: msg,
            })?;

        let marker = format!("<!-- parent-field-autolink:{} -->", self.issue);
        let label_required_missing = "issue-required-missing";
        let label_automation_failed = "automation-failed";
        let (repo_owner, repo_short_name) = split_repo_name(&repo_name);

        let issue_snapshot = issue_remote_snapshot_or_default(&repo_name, &self.issue);
        let issue_labels_raw = IssueRemoteSnapshot::issue_labels_raw(&issue_snapshot);
        let issue_title = &issue_snapshot.title;
        let issue_body = &issue_snapshot.body;
        let issue_state = &issue_snapshot.state;

        if self.issue.is_empty() {
            return Err(AutoLinkError::MissingIssueField);
        }

        if issue_state.is_empty() {
            return Err(AutoLinkError::UnableToReadIssue(self.issue.clone()));
        }

        let contract_errors =
            Validation::validate_content(issue_title, issue_body, &issue_labels_raw)
                .unwrap_or_default();
        if !contract_errors.is_empty() {
            let mut summary_lines = String::new();
            for entry in &contract_errors {
                summary_lines.push_str("- ");
                summary_lines.push_str(&entry.message);
                summary_lines.push('\n');
            }
            let help = format!(
                "Detected problems:\n\n{}\nExpected contract source: `.github/issue_required_fields.conf`.",
                summary_lines
            );
            return Err(AutoLinkError::ValidationError(help));
        }

        let parent_raw = match auto_link_extract_parent(issue_body) {
            Some(value) if !value.is_empty() => value,
            _ => {
                let help = "Expected format:\n\n- `Parent: #<issue_number>` for child issues\n\n- `Parent: none` for independent issues\n\n- `Parent: base` for cascade root issues\n\n- `Parent: epic` for epic umbrella issues";
                return Err(AutoLinkError::ValidationError(help.to_string()));
            }
        };

        let parent_raw_lc = parent_raw.to_lowercase();
        if parent_raw_lc == "none" || parent_raw_lc == "base" || parent_raw_lc == "epic" {
            run_auto_link_parent_none(
                &repo_name,
                &repo_owner,
                &repo_short_name,
                &self.issue,
                &parent_raw_lc,
                &marker,
                label_required_missing,
                label_automation_failed,
            );
            return Ok(());
        }

        if !is_issue_key(&parent_raw) {
            return Err(AutoLinkError::InvalidParentField(parent_raw));
        }

        run_auto_link_parent_link(
            &repo_name,
            &repo_owner,
            &repo_short_name,
            &self.issue,
            parent_raw.trim_start_matches('#'),
            &marker,
            label_required_missing,
            label_automation_failed,
        );

        Ok(())
    }
}
