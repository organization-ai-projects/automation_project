use crate::feedbacks::{FeedbackInput, InternalFeedbackEvent};

impl<'a> FeedbackInput<'a> {
    pub(crate) fn to_internal(&self) -> InternalFeedbackEvent<'a> {
        InternalFeedbackEvent {
            task_input: self.task_input.clone(),
            input: self.input.clone(),
            generated_output: self.generated_output.clone(),
            verdict: self.verdict.clone().into(),
            meta: self.meta.clone().into(),
        }
    }
}
