// projects/libraries/ai/src/feedbacks/internal/internal_feedback_verdict.rs
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::feedbacks::internal::{
    internal_feedback_input::InternalFeedbackInput, internal_feedback_meta::InternalFeedbackMeta,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum InternalFeedbackVerdict<'a> {
    Correct,
    Incorrect { expected_output: Cow<'a, str> },
    Partial { correction: Cow<'a, str> },
    Rejected,
    NoFeedback,
}

impl<'a> InternalFeedbackVerdict<'a> {
    pub fn stable_kind(&self) -> &'static str {
        match self {
            InternalFeedbackVerdict::Correct => "correct",
            InternalFeedbackVerdict::Rejected => "rejected",
            InternalFeedbackVerdict::NoFeedback => "no_feedback",
            InternalFeedbackVerdict::Incorrect { .. } => "incorrect",
            InternalFeedbackVerdict::Partial { .. } => "partial",
        }
    }

    pub fn stable_payload(&self) -> Option<&str> {
        match self {
            InternalFeedbackVerdict::Incorrect { expected_output } => Some(expected_output),
            InternalFeedbackVerdict::Partial { correction } => Some(correction),
            _ => None,
        }
    }

    /// Returns whether the verdict is positive (e.g., Correct).
    pub fn is_positive(&self) -> bool {
        matches!(self, InternalFeedbackVerdict::Correct)
    }

    /// Converts the verdict into an internal representation with metadata.
    pub fn to_internal_feedback(
        &self,
        task_input: Cow<'a, str>,
        input: Cow<'a, str>,
        generated_output: Cow<'a, str>,
        metadata: InternalFeedbackMeta,
    ) -> Option<InternalFeedbackInput<'a>> {
        match self {
            InternalFeedbackVerdict::Correct => Some(InternalFeedbackInput {
                task_input,
                input,
                generated_output,
                verdict: self.clone(),
                meta: metadata,
            }),
            InternalFeedbackVerdict::Incorrect { expected_output } => Some(InternalFeedbackInput {
                task_input,
                input,
                generated_output,
                verdict: InternalFeedbackVerdict::Incorrect {
                    expected_output: expected_output.clone(),
                },
                meta: metadata,
            }),
            InternalFeedbackVerdict::Partial { correction } => Some(InternalFeedbackInput {
                task_input,
                input,
                generated_output,
                verdict: InternalFeedbackVerdict::Partial {
                    correction: correction.clone(),
                },
                meta: metadata,
            }),
            InternalFeedbackVerdict::Rejected | InternalFeedbackVerdict::NoFeedback => None,
        }
    }
}
