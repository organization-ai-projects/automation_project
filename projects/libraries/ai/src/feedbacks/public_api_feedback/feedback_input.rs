// projects/libraries/ai/src/feedbacks/public_api_feedback/feedback_input.rs
use std::borrow::Cow;

use crate::feedbacks::public_api_feedback::{FeedbackMeta, FeedbackVerdict};

/// FeedbackInput is the main entry point / public DTO for feedback.
///
/// It is the only structure intended to be used by other modules in this crate.
#[derive(Debug, Clone)]
pub struct FeedbackInput<'a> {
    pub task_input: Cow<'a, str>,
    pub input: Cow<'a, str>,
    pub generated_output: Cow<'a, str>,
    pub verdict: FeedbackVerdict<'a>,
    pub meta: FeedbackMeta<'a>,
}

impl<'a> FeedbackInput<'a> {
    #[allow(dead_code)]
    pub(crate) fn correct(
        task_input: impl Into<Cow<'a, str>>,
        input: impl Into<Cow<'a, str>>,
        generated_output: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            task_input: task_input.into(),
            input: input.into(),
            generated_output: generated_output.into(),
            verdict: FeedbackVerdict::Correct,
            meta: FeedbackMeta::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn correct_with_meta(
        task_input: impl Into<Cow<'a, str>>,
        input: impl Into<Cow<'a, str>>,
        generated_output: impl Into<Cow<'a, str>>,
        meta: impl Into<FeedbackMeta<'a>>,
    ) -> Self {
        Self {
            task_input: task_input.into(),
            input: input.into(),
            generated_output: generated_output.into(),
            verdict: FeedbackVerdict::Correct,
            meta: meta.into(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn incorrect(
        task_input: impl Into<Cow<'a, str>>,
        input: impl Into<Cow<'a, str>>,
        generated_output: impl Into<Cow<'a, str>>,
        expected_output: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            task_input: task_input.into(),
            input: input.into(),
            generated_output: generated_output.into(),
            verdict: FeedbackVerdict::Incorrect {
                expected_output: expected_output.into(),
            },
            meta: FeedbackMeta::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn partial(
        task_input: impl Into<Cow<'a, str>>,
        input: impl Into<Cow<'a, str>>,
        generated_output: impl Into<Cow<'a, str>>,
        correction: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            task_input: task_input.into(),
            input: input.into(),
            generated_output: generated_output.into(),
            verdict: FeedbackVerdict::Partial {
                correction: correction.into(),
            },
            meta: FeedbackMeta::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn rejected(
        task_input: impl Into<Cow<'a, str>>,
        input: impl Into<Cow<'a, str>>,
        generated_output: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            task_input: task_input.into(),
            input: input.into(),
            generated_output: generated_output.into(),
            verdict: FeedbackVerdict::Rejected,
            meta: FeedbackMeta::new(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn no_feedback(
        task_input: impl Into<Cow<'a, str>>,
        input: impl Into<Cow<'a, str>>,
        generated_output: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            task_input: task_input.into(),
            input: input.into(),
            generated_output: generated_output.into(),
            verdict: FeedbackVerdict::NoFeedback,
            meta: FeedbackMeta::new(),
        }
    }
    #[allow(dead_code)]
    pub(crate) fn meta(mut self, meta: impl Into<FeedbackMeta<'a>>) -> Self {
        self.meta = meta.into();
        self
    }
}
