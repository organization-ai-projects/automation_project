// projects/libraries/ai/src/feedbacks/conversions/feedback_verdict/internal_feedback_verdict.rs
use crate::feedbacks::{FeedbackVerdict, InternalFeedbackVerdict};

impl<'a> From<FeedbackVerdict<'a>> for InternalFeedbackVerdict {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => InternalFeedbackVerdict::Correct,
            FeedbackVerdict::Rejected => InternalFeedbackVerdict::Rejected,
            FeedbackVerdict::Incorrect { expected_output } => InternalFeedbackVerdict::Incorrect {
                expected_output: expected_output.into_owned(),
            },
            FeedbackVerdict::Partial { correction } => InternalFeedbackVerdict::Partial {
                correction: correction.into_owned(),
            },
        }
    }
}
