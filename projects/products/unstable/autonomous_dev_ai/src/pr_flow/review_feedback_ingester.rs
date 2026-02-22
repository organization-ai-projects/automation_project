// projects/products/unstable/autonomous_dev_ai/src/pr_flow/review_feedback_ingester.rs
use super::{ReviewComment, ReviewOutcome};
use serde::{Deserialize, Serialize};

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
        if comments.is_empty() {
            return;
        }
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
        for c in &mut self.comments {
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
