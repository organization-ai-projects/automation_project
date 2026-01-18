// projects/libraries/ai/src/feedbacks/internal_feedback_event.rs
use std::borrow::Cow;

use crate::feedbacks::{InternalFeedbackMeta, InternalFeedbackRecord, InternalFeedbackVerdict};

#[derive(Debug, Clone)]
pub struct InternalFeedbackEvent<'a> {
    pub task_input: Cow<'a, str>,
    pub input: Cow<'a, str>,
    pub generated_output: Cow<'a, str>,
    pub verdict: InternalFeedbackVerdict,
    pub meta: InternalFeedbackMeta,
}

impl<'a> InternalFeedbackEvent<'a> {
    pub fn to_record(self) -> InternalFeedbackRecord<'a> {
        InternalFeedbackRecord::from_event(self)
    }
}
