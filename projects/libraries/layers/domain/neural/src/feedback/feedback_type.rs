// projects/libraries/layers/domain/neural/src/feedback/feedback_type.rs
use serde::{Deserialize, Serialize};

use crate::feedback::FeedbackMetadata;

/// Type of feedback the user can provide
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FeedbackType {
    /// Generated code was correct
    Correct {
        #[serde(flatten, default)]
        metadata: FeedbackMetadata,
    },

    /// Generated code was incorrect, here is the correct version
    Incorrect {
        expected_output: String,
        #[serde(flatten, default)]
        metadata: FeedbackMetadata,
    },

    /// Code was partially correct
    Partial {
        correction: String,
        #[serde(flatten, default)]
        metadata: FeedbackMetadata,
    },
}

impl FeedbackType {
    pub fn is_correct(&self) -> bool {
        matches!(self, FeedbackType::Correct { .. })
    }

    pub fn metadata(&self) -> &FeedbackMetadata {
        match self {
            FeedbackType::Correct { metadata }
            | FeedbackType::Incorrect { metadata, .. }
            | FeedbackType::Partial { metadata, .. } => metadata,
        }
    }
}
