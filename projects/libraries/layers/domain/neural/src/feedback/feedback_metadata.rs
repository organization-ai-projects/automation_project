// projects/libraries/layers/domain/neural/src/feedback/feedback_metadata.rs
use serde::{Deserialize, Serialize};

/// Common metadata for feedback types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeedbackMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Confidence in the feedback (optional)
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Reason or justification (optional)
    pub rationale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Source of the feedback
    pub source: Option<String>,
}
