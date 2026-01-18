// projects/libraries/ai/src/feedbacks/conversions/internal_feedback_verdict/feedback_type.rs
use crate::feedbacks::InternalFeedbackVerdict;
use neural::feedback::FeedbackType;

impl From<InternalFeedbackVerdict> for FeedbackType {
    fn from(verdict: InternalFeedbackVerdict) -> Self {
        match verdict {
            InternalFeedbackVerdict::Correct => FeedbackType::Correct {
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Rejected => FeedbackType::Incorrect {
                expected_output: "Rejected".to_string(),
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Incorrect { expected_output } => FeedbackType::Incorrect {
                expected_output,
                metadata: Default::default(),
            },
            InternalFeedbackVerdict::Partial { correction } => FeedbackType::Partial {
                correction,
                metadata: Default::default(),
            },
        }
    }
}
