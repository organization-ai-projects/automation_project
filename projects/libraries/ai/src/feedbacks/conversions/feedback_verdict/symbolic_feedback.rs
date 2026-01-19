// projects/libraries/ai/src/feedbacks/conversions/feedback_verdict/symbolic_feedback.rs
use crate::feedbacks::public_api_feedback::FeedbackVerdict;
use symbolic::feedback_symbolic::SymbolicFeedback;

impl<'a> From<FeedbackVerdict<'a>> for SymbolicFeedback {
    fn from(verdict: FeedbackVerdict<'a>) -> Self {
        match verdict {
            FeedbackVerdict::Correct => SymbolicFeedback::new(true, None),
            FeedbackVerdict::Rejected => SymbolicFeedback::new(false, None),
            FeedbackVerdict::NoFeedback => SymbolicFeedback::new(false, None),
            FeedbackVerdict::Incorrect { expected_output } => {
                SymbolicFeedback::new(false, Some(expected_output.into_owned()))
            }
            FeedbackVerdict::Partial { correction } => {
                SymbolicFeedback::new(false, Some(correction.into_owned()))
            }
        }
    }
}
