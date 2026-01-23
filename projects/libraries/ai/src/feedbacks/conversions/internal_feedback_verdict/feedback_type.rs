// projects/libraries/ai/src/feedbacks/conversions/internal_feedback_verdict/feedback_type.rs
use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use neural::feedback::FeedbackType;

const REJECTED: &str = "Rejected";
const NO_FEEDBACK: &str = "NoFeedback";

impl<'a> From<InternalFeedbackVerdict<'a>> for FeedbackType {
    fn from(verdict: InternalFeedbackVerdict<'a>) -> Self {
        match verdict {
            InternalFeedbackVerdict::Correct => FeedbackType::Correct {
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Rejected => FeedbackType::Incorrect {
                expected_output: REJECTED.to_string(),
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::NoFeedback => FeedbackType::Incorrect {
                expected_output: NO_FEEDBACK.to_string(),
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Incorrect { expected_output } => FeedbackType::Incorrect {
                expected_output: expected_output.into_owned(),
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Partial { correction } => FeedbackType::Partial {
                correction: correction.into_owned(),
                metadata: Default::default(),
            },
        }
    }
}
