// projects/libraries/neural/src/feedback/feedback_type.rs
use serde::{Deserialize, Serialize};

/// Métadonnées communes pour les types de feedback
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeedbackMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Confiance dans le feedback (optionnel)
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Raison ou justification (optionnel)
    pub rationale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Source du feedback
    pub source: Option<String>,
}

/// Type de feedback que l'utilisateur peut donner
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FeedbackType {
    /// Code généré était correct
    Correct {
        #[serde(flatten, default)]
        metadata: FeedbackMetadata,
    },

    /// Code généré était incorrect, voici la bonne version
    Incorrect {
        expected_output: String,
        #[serde(flatten, default)]
        metadata: FeedbackMetadata,
    },

    /// Code partiellement correct
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
