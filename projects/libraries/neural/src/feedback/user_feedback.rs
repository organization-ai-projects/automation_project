// projects/libraries/neural/src/feedback/user_feedback.rs
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use crate::feedback::FeedbackType;

/// structure du feadback utilisateur pour la partie neuronale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    /// Input original qui a produit la génération
    pub input: String,
    /// Output généré par le modèle
    pub generated_output: String,
    /// Type de feedback
    pub feedback_type: FeedbackType,

    /// Timestamp Unix en secondes (stable en sérialisation)
    pub timestamp_unix_secs: u64,
}

impl UserFeedback {
    pub fn new(
        input: impl Into<String>,
        generated_output: impl Into<String>,
        feedback_type: FeedbackType,
    ) -> Self {
        let timestamp_unix_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0); // Pas de log ici, laisse le caller gérer

        Self {
            input: input.into(),
            generated_output: generated_output.into(),
            feedback_type,
            timestamp_unix_secs,
        }
    }

    pub fn from_parts(input: &str, generated_output: &str, feedback_type: FeedbackType) -> Self {
        let timestamp_unix_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            input: input.to_string(),
            generated_output: generated_output.to_string(),
            feedback_type,
            timestamp_unix_secs,
        }
    }

    pub fn formatted_timestamp(&self) -> String {
        common::format_timestamp(self.timestamp_unix_secs)
    }
}

impl Hash for UserFeedback {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.input.hash(state);
        self.generated_output.hash(state);
        self.timestamp_unix_secs.hash(state);
    }
}
