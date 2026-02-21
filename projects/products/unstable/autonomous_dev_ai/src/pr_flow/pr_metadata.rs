//projects/products/unstable/autonomous_dev_ai/src/pr_flow/pr_metadata.rs
use super::{CiStatus, IssueComplianceStatus};
use serde::{Deserialize, Serialize};

/// Metadata for a pull request managed by the autonomous agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrMetadata {
    /// PR number (None until the PR has been created on the remote).
    pub pr_number: Option<u64>,
    pub title: String,
    pub body: String,
    /// Branches this PR closes (issue numbers as strings).
    pub closes_issues: Vec<String>,
    pub ci_status: CiStatus,
    pub policy_compliant: bool,
    pub issue_compliance: IssueComplianceStatus,
}

impl PrMetadata {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            pr_number: None,
            title: title.into(),
            body: body.into(),
            closes_issues: Vec::new(),
            ci_status: CiStatus::Unknown,
            policy_compliant: false,
            issue_compliance: IssueComplianceStatus::Unknown,
        }
    }

    /// Add a `Closes #N` reference to the PR body.
    pub fn close_issue(&mut self, issue_number: &str) {
        if !self.closes_issues.contains(&issue_number.to_string()) {
            self.closes_issues.push(issue_number.to_string());
        }
    }

    /// Render a deterministic PR description that embeds issue closure refs.
    pub fn render_body(&self) -> String {
        let closes_lines: Vec<String> = self
            .closes_issues
            .iter()
            .map(|n| format!("Closes #{n}"))
            .collect();

        let footer = if closes_lines.is_empty() {
            String::new()
        } else {
            format!("\n\n---\n{}", closes_lines.join("\n"))
        };

        format!("{}{}", self.body, footer)
    }
}
