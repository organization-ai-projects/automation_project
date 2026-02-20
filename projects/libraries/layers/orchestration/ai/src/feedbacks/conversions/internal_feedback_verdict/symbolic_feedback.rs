// projects/libraries/layers/orchestration/ai/src/feedbacks/conversions/internal_feedback_verdict/symbolic_feedback.rs
use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use symbolic::feedback_symbolic::SymbolicFeedback;

impl<'a> From<InternalFeedbackVerdict<'a>> for SymbolicFeedback {
    fn from(verdict: InternalFeedbackVerdict<'a>) -> Self {
        let is_positive = verdict.is_positive();
        let payload = verdict.stable_payload().map(|p| p.to_owned());
        SymbolicFeedback::new(is_positive, payload)
    }
}
