// projects/libraries/layers/orchestration/ai/src/feedbacks/conversions/feedback_verdict/internal_feedback_verdict.rs
use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use crate::feedbacks::public_api_feedback::feedback_verdict::FeedbackVerdict;

/// Converts a public verdict (`FeedbackVerdict`) to its internal equivalent (`InternalFeedbackVerdict`).
impl<'a> From<FeedbackVerdict<'a>> for InternalFeedbackVerdict<'a> {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => InternalFeedbackVerdict::Correct,
            FeedbackVerdict::Rejected => InternalFeedbackVerdict::Rejected,
            FeedbackVerdict::NoFeedback => InternalFeedbackVerdict::NoFeedback,
            FeedbackVerdict::Incorrect { expected_output } => {
                InternalFeedbackVerdict::Incorrect { expected_output }
            }
            FeedbackVerdict::Partial { correction } => {
                InternalFeedbackVerdict::Partial { correction }
            }
        }
    }
}
