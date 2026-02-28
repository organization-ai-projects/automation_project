use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewVerdict {
    Approve,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerVerdict {
    pub reviewer_id: String,
    pub specialty: String,
    pub verdict: ReviewVerdict,
    pub confidence: u8,
    pub weight: u8,
    pub reason_codes: Vec<String>,
}
