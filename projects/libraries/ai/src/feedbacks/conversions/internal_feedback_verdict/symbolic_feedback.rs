// projects/libraries/ai/src/feedbacks/conversions/internal_feedback_verdict/symbolic_feedback.rs
use crate::feedbacks::InternalFeedbackVerdict;
use symbolic::feedback_symbolic::SymbolicFeedback;

impl From<InternalFeedbackVerdict> for SymbolicFeedback {
    fn from(verdict: InternalFeedbackVerdict) -> Self {
        match verdict {
            InternalFeedbackVerdict::Correct => SymbolicFeedback::new(true, None),
            InternalFeedbackVerdict::Rejected => SymbolicFeedback::new(false, None),
            InternalFeedbackVerdict::Incorrect { expected_output } => {
                SymbolicFeedback::new(false, Some(expected_output))
            }
            InternalFeedbackVerdict::Partial { correction } => {
                SymbolicFeedback::new(false, Some(correction))
            }
        }
    }
}
