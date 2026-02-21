//projects/products/unstable/autonomous_dev_ai/src/pr_flow/review_outcome.rs
use serde::{Deserialize, Serialize};

/// Outcome of a review iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewOutcome {
    Approved,
    ChangesRequested,
    Timeout,
}
