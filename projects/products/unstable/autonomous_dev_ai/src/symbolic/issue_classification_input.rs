// projects/products/unstable/autonomous_dev_ai/src/symbolic/issue_classification_input.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueClassificationInput {
    pub labels: Vec<String>,
    pub title: String,
    pub body: String,
}
