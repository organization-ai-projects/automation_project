use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Métadonnées publiques (API). Stable et extensible.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeedbackMeta<'a> {
    pub confidence: Option<f32>,
    pub rationale: Option<Cow<'a, str>>,
    pub source: Option<Cow<'a, str>>,
}

impl<'a> FeedbackMeta<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn confidence(mut self, v: f32) -> Self {
        self.confidence = Some(v);
        self
    }

    pub fn rationale(mut self, v: impl Into<Cow<'a, str>>) -> Self {
        self.rationale = Some(v.into());
        self
    }

    pub fn source(mut self, v: impl Into<Cow<'a, str>>) -> Self {
        self.source = Some(v.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.confidence.is_none() && self.rationale.is_none() && self.source.is_none()
    }
}

/// Verdict public (API). Impossible d’être ambigu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackVerdict<'a> {
    Correct,
    Incorrect { expected_output: Cow<'a, str> },
    Partial { correction: Cow<'a, str> },
    Rejected,
}

/// Requête publique complète (API).
#[derive(Debug, Clone)]
pub struct FeedbackInput<'a> {
    pub task_input: Cow<'a, str>,
    pub input: Cow<'a, str>,
    pub generated_output: Cow<'a, str>,
    pub verdict: FeedbackVerdict<'a>,
    pub meta: FeedbackMeta<'a>,
}

impl<'a> FeedbackInput<'a> {
    pub fn correct(
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

    pub fn incorrect_expected(
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

    pub fn partial_correction(
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

    pub fn rejected(
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

    pub fn meta(mut self, meta: FeedbackMeta<'a>) -> Self {
        self.meta = meta;
        self
    }
}
