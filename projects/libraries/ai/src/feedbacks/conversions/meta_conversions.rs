// projects/libraries/ai/src/feedbacks/conversions/meta_conversions.rs
use crate::feedbacks::{FeedbackMeta, InternalFeedbackMeta};

impl<'a> From<FeedbackMeta<'a>> for InternalFeedbackMeta {
    fn from(meta: FeedbackMeta<'a>) -> Self {
        InternalFeedbackMeta {
            confidence: meta.confidence,
            rationale: meta.rationale.map(|s| s.into_owned()),
            source: meta.source.map(|s| s.into_owned()),
        }
    }
}
