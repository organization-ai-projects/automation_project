use serde::{Deserialize, Serialize};

/// Describes how aggressive or passive an AI faction plays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiProfile {
    /// Probability (0..=100) of choosing Move over Hold.
    pub move_probability: u32,
}

impl Default for AiProfile {
    fn default() -> Self {
        Self {
            move_probability: 50,
        }
    }
}
