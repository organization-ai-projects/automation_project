// projects/libraries/ai/src/feedbacks/internal/internal_feedback_meta.rs
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::feedbacks::public_api_feedback::FeedbackMeta;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct InternalFeedbackMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confidence: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) rationale: Option<Cow<'static, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) source: Option<Cow<'static, str>>,
}

impl InternalFeedbackMeta {
    pub fn is_empty(&self) -> bool {
        self.confidence.is_none() && self.rationale.is_none() && self.source.is_none()
    }
}

impl<'a> From<FeedbackMeta<'a>> for InternalFeedbackMeta {
    fn from(meta: FeedbackMeta<'a>) -> Self {
        InternalFeedbackMeta {
            confidence: meta.confidence,
            rationale: meta.rationale.map(|s| Cow::Owned(s.into_owned())),
            source: meta.source.map(|s| Cow::Owned(s.into_owned())),
        }
    }
}
