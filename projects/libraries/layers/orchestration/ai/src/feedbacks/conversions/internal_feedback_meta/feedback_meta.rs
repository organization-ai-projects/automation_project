use crate::feedbacks::internal::internal_feedback_meta::InternalFeedbackMeta;
use crate::feedbacks::public_api_feedback::feedback_metadata::FeedbackMeta;
use std::borrow::Cow;

/// Converts an internal feedback meta (`InternalFeedbackMeta`) to its public equivalent (`FeedbackMeta`).
impl<'a> From<InternalFeedbackMeta> for FeedbackMeta<'a> {
    fn from(meta: InternalFeedbackMeta) -> Self {
        FeedbackMeta {
            confidence: meta.confidence,
            rationale: meta.rationale.map(|s| match s {
                Cow::Borrowed(inner) => Cow::Borrowed(inner),
                Cow::Owned(inner) => Cow::Owned(inner),
            }),
            source: meta.source.map(|s| match s {
                Cow::Borrowed(inner) => Cow::Borrowed(inner),
                Cow::Owned(inner) => Cow::Owned(inner),
            }),
        }
    }
}
