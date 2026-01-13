// projects/libraries/ai/src/feedback_record.rs
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::feedbacks::{InternalFeedbackMeta, InternalFeedbackVerdict};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalFeedbackRecord<'a> {
    pub task_input: Cow<'a, str>,
    pub input: Cow<'a, str>,
    pub generated_output: Cow<'a, str>,
    pub verdict: InternalFeedbackVerdict,
    #[serde(default, skip_serializing_if = "InternalFeedbackMeta::is_empty")]
    pub meta: InternalFeedbackMeta,
}

impl<'a> InternalFeedbackRecord<'a> {
    pub fn from_event(event: InternalFeedbackEvent<'a>) -> Self {
        Self {
            task_input: event.task_input,
            input: event.input,
            generated_output: event.generated_output,
            verdict: event.verdict,
            meta: event.meta,
        }
    }
}

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
