// projects/products/unstable/autonomous_dev_ai/src/pr_flow/pr_metadata.rs
use super::{CiStatus, IssueComplianceStatus};
use crate::ids::{IssueNumber, PrNumber};
use serde::{Deserialize, Serialize};

/// Metadata for a pull request managed by the autonomous agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrMetadata {
    /// PR number (None until the PR has been created on the remote).
    pub pr_number: Option<PrNumber>,
    pub title: String,
    pub body: String,
    /// Branches this PR closes (issue numbers as strings).
    pub closes_issues: Vec<IssueNumber>,
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
    pub fn close_issue(&mut self, issue_number: IssueNumber) {
        if !self.closes_issues.contains(&issue_number) {
            self.closes_issues.push(issue_number);
        }
    }

    /// Render a deterministic PR description that embeds issue closure refs.
    pub fn render_body(&self) -> String {
        let closure_keyword =
            std::env::var("AUTONOMOUS_CLOSURE_KEYWORD").unwrap_or_else(|_| "Closes".to_string());
        let neutralize = !matches!(self.issue_compliance, IssueComplianceStatus::Compliant);
        let closes_lines: Vec<String> = self
            .closes_issues
            .iter()
            .map(|n| {
                if neutralize {
                    format!("{closure_keyword} Rejected #{n}")
                } else {
                    format!("{closure_keyword} #{n}")
                }
            })
            .collect();

        let footer = if closes_lines.is_empty() {
            String::new()
        } else {
            format!("\n\n---\n{}", closes_lines.join("\n"))
        };

        format!("{}{}", self.body, footer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn render_body_keeps_closure_keyword_when_issue_is_compliant() {
        let _guard = ENV_LOCK.lock().expect("env lock poisoned");
        let mut metadata = PrMetadata::new("title", "body");
        metadata.issue_compliance = IssueComplianceStatus::Compliant;
        metadata.close_issue(IssueNumber::new(42).expect("valid issue number"));

        // SAFETY: test controls process env while holding ENV_LOCK.
        unsafe { std::env::set_var("AUTONOMOUS_CLOSURE_KEYWORD", "Closes") };
        let rendered = metadata.render_body();
        // SAFETY: cleanup after test while holding ENV_LOCK.
        unsafe { std::env::remove_var("AUTONOMOUS_CLOSURE_KEYWORD") };
        assert!(rendered.contains("Closes #42"), "rendered body: {rendered}");
    }

    #[test]
    fn render_body_neutralizes_closure_when_issue_non_compliant() {
        let _guard = ENV_LOCK.lock().expect("env lock poisoned");
        let mut metadata = PrMetadata::new("title", "body");
        metadata.issue_compliance = IssueComplianceStatus::NonCompliant {
            reason: "missing Parent".to_string(),
        };
        metadata.close_issue(IssueNumber::new(51).expect("valid issue number"));

        // SAFETY: test controls process env while holding ENV_LOCK.
        unsafe { std::env::set_var("AUTONOMOUS_CLOSURE_KEYWORD", "Closes") };
        let rendered = metadata.render_body();
        // SAFETY: cleanup after test while holding ENV_LOCK.
        unsafe { std::env::remove_var("AUTONOMOUS_CLOSURE_KEYWORD") };
        assert!(
            rendered.contains("Closes Rejected #51"),
            "rendered body: {rendered}"
        );
    }

    #[test]
    fn render_body_preserves_custom_keyword_while_neutralizing() {
        let _guard = ENV_LOCK.lock().expect("env lock poisoned");
        let mut metadata = PrMetadata::new("title", "body");
        metadata.issue_compliance = IssueComplianceStatus::NonCompliant {
            reason: "missing Parent".to_string(),
        };
        metadata.close_issue(IssueNumber::new(64).expect("valid issue number"));

        // SAFETY: test process controls env for this case only.
        unsafe { std::env::set_var("AUTONOMOUS_CLOSURE_KEYWORD", "Fixes") };
        let rendered = metadata.render_body();
        // SAFETY: cleanup after test.
        unsafe { std::env::remove_var("AUTONOMOUS_CLOSURE_KEYWORD") };

        assert!(
            rendered.contains("Fixes Rejected #64"),
            "rendered body: {rendered}"
        );
    }
}
