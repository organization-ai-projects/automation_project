use common::format_timestamp;
use serde::{Deserialize, Serialize};

use crate::feedback::FeedbackType;

/// Structure du feedback utilisateur
#[derive(Debug, Clone)]
pub struct UserFeedback {
    /// Input original qui a produit la génération
    pub input: String,
    /// Output généré par le modèle
    pub generated_output: String,
    /// Type de feedback
    pub feedback_type: FeedbackType,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

impl UserFeedback {
    pub fn formatted_timestamp(&self) -> String {
        format_timestamp(
            self.timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
}

impl Serialize for UserFeedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("UserFeedback", 4)?;
        state.serialize_field("input", &self.input)?;
        state.serialize_field("generated_output", &self.generated_output)?;
        state.serialize_field("feedback_type", &format!("{:?}", self.feedback_type))?;
        state.serialize_field(
            "timestamp",
            &self
                .timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for UserFeedback {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Implémentation simplifiée
        todo!("Implement deserialize for UserFeedback")
    }
}
