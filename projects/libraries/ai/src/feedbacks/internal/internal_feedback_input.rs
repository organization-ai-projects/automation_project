// projects/libraries/ai/src/feedbacks/internal/internal_feedback_record.rs
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::feedbacks::internal::{
    internal_feedback_meta::InternalFeedbackMeta,
    internal_feedback_verdict::InternalFeedbackVerdict,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InternalFeedbackInput<'a> {
    pub(crate) task_input: Cow<'a, str>,
    pub(crate) input: Cow<'a, str>,
    pub(crate) generated_output: Cow<'a, str>,
    pub(crate) verdict: InternalFeedbackVerdict<'a>,
    #[serde(default, skip_serializing_if = "InternalFeedbackMeta::is_empty")]
    pub(crate) meta: InternalFeedbackMeta,
}
