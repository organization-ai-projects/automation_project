// projects/libraries/ai/src/feedbacks/conversions/feedback_verdict/feedback_type.rs
use crate::feedbacks::FeedbackVerdict;
use neural::feedback::FeedbackType;

impl<'a> From<FeedbackVerdict<'a>> for FeedbackType {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => FeedbackType::Correct {
                metadata: Default::default(),
            },
            FeedbackVerdict::Rejected => FeedbackType::Incorrect {
                expected_output: "Rejected".to_string(),
                metadata: Default::default(),
            },
            FeedbackVerdict::Incorrect { expected_output } => FeedbackType::Incorrect {
                expected_output: expected_output.into_owned(),
                metadata: Default::default(),
            },
            FeedbackVerdict::Partial { correction } => FeedbackType::Partial {
                correction: correction.into_owned(),
                metadata: Default::default(),
            },
        }
    }
}
