// projects/products/unstable/autonomous_dev_ai/src/pr_flow/review_comment.rs
use serde::{Deserialize, Serialize};

/// A single piece of review feedback from a reviewer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewComment {
    pub reviewer: String,
    pub body: String,
    pub resolved: bool,
}
