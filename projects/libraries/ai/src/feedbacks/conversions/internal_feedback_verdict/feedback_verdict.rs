use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use crate::feedbacks::public_api_feedback::feedback_verdict::FeedbackVerdict;

/// Converts an internal verdict (`InternalFeedbackVerdict`) to its public equivalent (`FeedbackVerdict`).
impl<'a> From<InternalFeedbackVerdict<'a>> for FeedbackVerdict<'a> {
    fn from(verdict: InternalFeedbackVerdict<'a>) -> Self {
        match verdict {
            InternalFeedbackVerdict::Correct => FeedbackVerdict::Correct,
            InternalFeedbackVerdict::Rejected => FeedbackVerdict::Rejected,
            InternalFeedbackVerdict::NoFeedback => FeedbackVerdict::NoFeedback,
            InternalFeedbackVerdict::Incorrect { expected_output } => {
                FeedbackVerdict::Incorrect { expected_output }
            }
            InternalFeedbackVerdict::Partial { correction } => {
                FeedbackVerdict::Partial { correction }
            }
        }
    }
}
