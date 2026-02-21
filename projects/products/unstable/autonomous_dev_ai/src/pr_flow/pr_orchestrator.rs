// projects/products/unstable/autonomous_dev_ai/src/pr_flow/pr_orchestrator.rs
use super::{
    CiStatus, IssueComplianceStatus, MergeReadiness, MergeReadinessChecker, PrMetadata,
    ReviewComment, ReviewFeedbackIngester, ReviewOutcome,
};
use crate::ids::PrNumber;
use serde::{Deserialize, Serialize};

/// Orchestrates the full PR lifecycle: open -> review loop -> merge readiness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrOrchestrator {
    pub metadata: PrMetadata,
    pub review_ingester: ReviewFeedbackIngester,
}

impl PrOrchestrator {
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        max_review_iterations: usize,
    ) -> Self {
        Self {
            metadata: PrMetadata::new(title, body),
            review_ingester: ReviewFeedbackIngester::new(max_review_iterations),
        }
    }

    /// "Open" the PR (in a real integration this would call the GitHub API).
    pub fn open(&mut self, pr_number: PrNumber) {
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
