// projects/libraries/ai/src/feedback_verdict.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InternalFeedbackVerdict {
    Correct,
    Incorrect { expected_output: String },
    Partial { correction: String },
    Rejected,
}

impl InternalFeedbackVerdict {
    pub fn stable_kind(&self) -> &'static str {
        match self {
            InternalFeedbackVerdict::Correct => "correct",
            InternalFeedbackVerdict::Rejected => "rejected",
            InternalFeedbackVerdict::Incorrect { .. } => "incorrect",
            InternalFeedbackVerdict::Partial { .. } => "partial",
        }
    }

    pub fn stable_payload(&self) -> Option<&str> {
        match self {
            InternalFeedbackVerdict::Incorrect { expected_output } => Some(expected_output),
            InternalFeedbackVerdict::Partial { correction } => Some(correction),
            _ => None,
        }
    }
}
