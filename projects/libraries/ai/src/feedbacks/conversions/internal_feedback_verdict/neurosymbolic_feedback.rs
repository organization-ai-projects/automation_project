use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use crate::feedbacks::neurosymbolic_feedback::NeurosymbolicFeedback;
use neural::feedback::FeedbackType;
use symbolic::feedback_symbolic::SymbolicFeedback;

impl<'a> From<InternalFeedbackVerdict<'a>> for NeurosymbolicFeedback {
    fn from(verdict: InternalFeedbackVerdict<'a>) -> Self {
        let symbolic_feedback: SymbolicFeedback = verdict.clone().into();
        let neural_feedback: FeedbackType = verdict.into();

        NeurosymbolicFeedback::new(
            symbolic_feedback,
            neural_feedback,
            None, // Metadata can be added here if needed
        )
    }
}
