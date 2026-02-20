// projects/libraries/layers/orchestration/ai/src/feedbacks/conversions/internal_feedback_verdict/feedback_type.rs
use crate::feedbacks::internal::internal_feedback_verdict::InternalFeedbackVerdict;
use neural::feedback::FeedbackType;

impl<'a> From<InternalFeedbackVerdict<'a>> for FeedbackType {
    fn from(verdict: InternalFeedbackVerdict<'a>) -> Self {
        // Handle Rejected and NoFeedback directly since to_internal_feedback returns None for them
        match &verdict {
            InternalFeedbackVerdict::Rejected => {
                return FeedbackType::Incorrect {
                    expected_output: "Rejected".to_string(),
                    metadata: Default::default(),
                };
            }
            InternalFeedbackVerdict::NoFeedback => {
                return FeedbackType::Incorrect {
                    expected_output: "NoFeedback".to_string(),
                    metadata: Default::default(),
                };
            }
            _ => {}
        }

        match verdict.to_internal_feedback(
            "task_input_placeholder".into(),
            "input_placeholder".into(),
            "generated_output_placeholder".into(),
            Default::default(),
        ) {
            Some(internal_feedback) => match internal_feedback.verdict {
                InternalFeedbackVerdict::Correct => FeedbackType::Correct {
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
                // These cases are now handled above, but keep for completeness
                InternalFeedbackVerdict::Rejected | InternalFeedbackVerdict::NoFeedback => {
                    unreachable!("Rejected and NoFeedback are handled before to_internal_feedback")
                }
            },
            None => FeedbackType::Incorrect {
                expected_output: "Invalid Feedback".to_string(),
                metadata: Default::default(),
            },
        }
    }
}
