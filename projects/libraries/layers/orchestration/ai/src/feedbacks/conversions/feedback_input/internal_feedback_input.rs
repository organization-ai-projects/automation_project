use crate::feedbacks::internal::internal_feedback_input::InternalFeedbackInput;
use crate::feedbacks::public_api_feedback::feedback_input::FeedbackInput;

/// Converts a public feedback input (`FeedbackInput`) to its internal equivalent (`InternalFeedbackInput`).
impl<'a> From<&FeedbackInput<'a>> for InternalFeedbackInput<'a> {
    fn from(input: &FeedbackInput<'a>) -> Self {
        InternalFeedbackInput {
            task_input: input.task_input.clone(),
            input: input.input.clone(),
            generated_output: input.generated_output.clone(),
            verdict: input.verdict.clone().into(),
            meta: input.meta.clone().into(),
        }
    }
}
