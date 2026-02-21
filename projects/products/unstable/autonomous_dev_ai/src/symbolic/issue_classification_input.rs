use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueClassificationInput {
    pub labels: Vec<String>,
    pub title: String,
    pub body: String,
}
