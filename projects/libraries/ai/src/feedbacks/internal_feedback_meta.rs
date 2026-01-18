// projects/libraries/ai/src/feedback_meta.rs
use serde::{Deserialize, Serialize};

use crate::feedbacks::FeedbackMeta;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InternalFeedbackMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
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
            rationale: meta.rationale.map(|s| s.into_owned()),
            source: meta.source.map(|s| s.into_owned()),
        }
    }
}
