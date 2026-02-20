// projects/products/unstable/autonomous_dev_ai/src/pr_flow/mod.rs

//! Autonomous PR/review/merge orchestration.
//!
//! Handles the full PR lifecycle: opening/updating PRs with deterministic
//! metadata, iterative review feedback ingestion, and merge readiness checks
//! that aggregate CI status, policy status, and issue compliance gates.

use serde::{Deserialize, Serialize};

// ─── PR Metadata ─────────────────────────────────────────────────────────────

/// Status of a CI check for a given PR.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CiStatus {
    Pending,
    Passing,
    Failing,
    Unknown,
}

/// Compliance state relative to repository rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueComplianceStatus {
    Compliant,
    NonCompliant { reason: String },
    Unknown,
}

/// Aggregated merge-readiness verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeReadiness {
    Ready,
    NotReady { reasons: Vec<String> },
}

impl MergeReadiness {
    pub fn is_ready(&self) -> bool {
        matches!(self, MergeReadiness::Ready)
    }
}

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

// ─── Review Feedback ─────────────────────────────────────────────────────────

/// A single piece of review feedback from a reviewer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewComment {
    pub reviewer: String,
    pub body: String,
    pub resolved: bool,
}

/// Outcome of a review iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewOutcome {
    Approved,
    ChangesRequested,
    Timeout,
}

// ─── Review Feedback Ingester ────────────────────────────────────────────────

/// Ingests review comments and drives iterative fix loops.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewFeedbackIngester {
    pub max_iterations: usize,
    pub iteration: usize,
    pub comments: Vec<ReviewComment>,
}

impl ReviewFeedbackIngester {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            max_iterations,
            iteration: 0,
            comments: Vec::new(),
        }
    }

    /// Ingest a batch of new review comments.
    pub fn ingest(&mut self, comments: Vec<ReviewComment>) {
        self.comments.extend(comments);
        self.iteration = self.iteration.saturating_add(1);
    }

    /// Check whether the iteration budget has been exceeded.
    pub fn budget_exceeded(&self) -> bool {
        self.iteration >= self.max_iterations
    }

    /// Check whether all comments are resolved.
    pub fn all_resolved(&self) -> bool {
        self.comments.iter().all(|c| c.resolved)
    }

    /// Resolve a comment by reviewer + body substring.
    pub fn resolve(&mut self, reviewer: &str) {
        for c in self.comments.iter_mut() {
            if c.reviewer == reviewer {
                c.resolved = true;
            }
        }
    }

    /// Determine the review outcome.
    pub fn outcome(&self) -> ReviewOutcome {
        if self.budget_exceeded() {
            return ReviewOutcome::Timeout;
        }
        if self.all_resolved() {
            ReviewOutcome::Approved
        } else {
            ReviewOutcome::ChangesRequested
        }
    }

    /// Unresolved comment bodies (for agent action planning).
    pub fn pending_feedback(&self) -> Vec<&ReviewComment> {
        self.comments.iter().filter(|c| !c.resolved).collect()
    }
}

// ─── Merge Readiness Checker ─────────────────────────────────────────────────

/// Aggregates CI status, policy status, and issue compliance to determine
/// whether a PR is ready to merge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeReadinessChecker;

impl MergeReadinessChecker {
    pub fn check(pr: &PrMetadata) -> MergeReadiness {
        let mut reasons = Vec::new();

        if pr.ci_status != CiStatus::Passing {
            reasons.push(format!("CI is not passing (status: {:?})", pr.ci_status));
        }

        if !pr.policy_compliant {
            reasons.push("Policy compliance check has not passed".to_string());
        }

        match &pr.issue_compliance {
            IssueComplianceStatus::Compliant => {}
            IssueComplianceStatus::NonCompliant { reason } => {
                reasons.push(format!("Issue compliance failed: {reason}"));
            }
            IssueComplianceStatus::Unknown => {
                reasons.push("Issue compliance status is unknown".to_string());
            }
        }

        if reasons.is_empty() {
            MergeReadiness::Ready
        } else {
            MergeReadiness::NotReady { reasons }
        }
    }
}

// ─── PR Orchestrator ─────────────────────────────────────────────────────────

/// Orchestrates the full PR lifecycle: open → review loop → merge readiness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrOrchestrator {
    pub metadata: PrMetadata,
    pub review_ingester: ReviewFeedbackIngester,
}

impl PrOrchestrator {
    pub fn new(title: impl Into<String>, body: impl Into<String>, max_review_iterations: usize) -> Self {
        Self {
            metadata: PrMetadata::new(title, body),
            review_ingester: ReviewFeedbackIngester::new(max_review_iterations),
        }
    }

    /// "Open" the PR (in a real integration this would call the GitHub API).
    pub fn open(&mut self, pr_number: u64) {
        self.metadata.pr_number = Some(pr_number);
        tracing::info!("PR #{} opened: {}", pr_number, self.metadata.title);
    }

    /// Update the PR body with fresh content.
    pub fn update_body(&mut self, new_body: impl Into<String>) {
        self.metadata.body = new_body.into();
        tracing::info!("PR body updated");
    }

    /// Mark CI status.
    pub fn set_ci_status(&mut self, status: CiStatus) {
        self.metadata.ci_status = status;
    }

    /// Mark policy compliance.
    pub fn set_policy_compliant(&mut self, compliant: bool) {
        self.metadata.policy_compliant = compliant;
    }

    /// Mark issue compliance.
    pub fn set_issue_compliance(&mut self, status: IssueComplianceStatus) {
        self.metadata.issue_compliance = status;
    }

    /// Check merge readiness.
    pub fn merge_readiness(&self) -> MergeReadiness {
        MergeReadinessChecker::check(&self.metadata)
    }

    /// Ingest review feedback and return the current outcome.
    pub fn ingest_review(&mut self, comments: Vec<ReviewComment>) -> ReviewOutcome {
        self.review_ingester.ingest(comments);
        self.review_ingester.outcome()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pr_metadata_render_body_with_closes() {
        let mut meta = PrMetadata::new("Fix bug", "Body content");
        meta.close_issue("647");
        meta.close_issue("651");
        let body = meta.render_body();
        assert!(body.contains("Closes #647"));
        assert!(body.contains("Closes #651"));
        assert!(body.contains("Body content"));
    }

    #[test]
    fn test_merge_readiness_all_green() {
        let mut meta = PrMetadata::new("Fix bug", "Body");
        meta.ci_status = CiStatus::Passing;
        meta.policy_compliant = true;
        meta.issue_compliance = IssueComplianceStatus::Compliant;
        assert!(MergeReadinessChecker::check(&meta).is_ready());
    }

    #[test]
    fn test_merge_readiness_ci_failing() {
        let mut meta = PrMetadata::new("Fix bug", "Body");
        meta.ci_status = CiStatus::Failing;
        meta.policy_compliant = true;
        meta.issue_compliance = IssueComplianceStatus::Compliant;
        let verdict = MergeReadinessChecker::check(&meta);
        assert!(!verdict.is_ready());
        if let MergeReadiness::NotReady { reasons } = verdict {
            assert!(reasons.iter().any(|r| r.contains("CI")));
        }
    }

    #[test]
    fn test_review_feedback_ingester_flow() {
        let mut ingester = ReviewFeedbackIngester::new(3);
        ingester.ingest(vec![ReviewComment {
            reviewer: "alice".to_string(),
            body: "Please fix the typo".to_string(),
            resolved: false,
        }]);

        assert_eq!(ingester.outcome(), ReviewOutcome::ChangesRequested);
        assert!(!ingester.all_resolved());

        ingester.resolve("alice");
        assert_eq!(ingester.outcome(), ReviewOutcome::Approved);
    }

    #[test]
    fn test_review_feedback_ingester_timeout() {
        let mut ingester = ReviewFeedbackIngester::new(2);
        for _ in 0..3 {
            ingester.ingest(vec![ReviewComment {
                reviewer: "bot".to_string(),
                body: "still unresolved".to_string(),
                resolved: false,
            }]);
        }
        assert_eq!(ingester.outcome(), ReviewOutcome::Timeout);
    }

    #[test]
    fn test_pr_orchestrator_lifecycle() {
        let mut orch = PrOrchestrator::new("Add feature", "This PR adds…", 5);
        orch.open(42);
        orch.set_ci_status(CiStatus::Passing);
        orch.set_policy_compliant(true);
        orch.set_issue_compliance(IssueComplianceStatus::Compliant);
        orch.metadata.close_issue("647");

        assert!(orch.merge_readiness().is_ready());
        assert!(orch.metadata.render_body().contains("Closes #647"));
    }
}
