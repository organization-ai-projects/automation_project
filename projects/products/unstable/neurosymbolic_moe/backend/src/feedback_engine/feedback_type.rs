use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeedbackType {
    Positive,
    Negative,
    Correction,
    Suggestion,
}
