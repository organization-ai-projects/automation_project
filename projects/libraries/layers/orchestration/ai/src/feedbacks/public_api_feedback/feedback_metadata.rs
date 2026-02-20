// projects/libraries/layers/orchestration/ai/src/feedbacks/public_api_feedback/feedback_metadata.rs
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// FeedbackMeta is an internal structure used only by FeedbackInput.
/// It should not be used directly outside this context.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeedbackMeta<'a> {
    pub confidence: Option<f32>,
    pub rationale: Option<Cow<'a, str>>,
    pub source: Option<Cow<'a, str>>,
}

impl<'a> FeedbackMeta<'a> {
    pub(crate) fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub(crate) fn confidence(mut self, v: f32) -> Self {
        self.confidence = Some(v);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn rationale(mut self, v: impl Into<Cow<'a, str>>) -> Self {
        self.rationale = Some(v.into());
        self
    }

    #[allow(dead_code)]
    pub(crate) fn source(mut self, v: impl Into<Cow<'a, str>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.confidence.is_none() && self.rationale.is_none() && self.source.is_none()
    }
}
