// projects/libraries/layers/orchestration/ai/src/feedbacks/public_api_feedback/feedback_verdict.rs
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// FeedbackVerdict is an internal enum used only by FeedbackInput.
/// It should not be used directly outside this context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackVerdict<'a> {
    /// The output is correct.
    Correct,
    /// The output is incorrect with an expected answer.
    Incorrect { expected_output: Cow<'a, str> },
    /// The output is partially correct with a suggested correction.
    Partial { correction: Cow<'a, str> },
    /// The evaluator refuses to judge (explicit refusal).
    Rejected,
    /// The evaluator provides no feedback (no judgment given).
    NoFeedback,
}
