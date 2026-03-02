use serde::{Deserialize, Serialize};

/// AI personality/behaviour profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProfile {
    pub name: String,
    pub aggression: f64,
    pub diplomacy_bias: f64,
    pub economic_focus: f64,
}

impl Default for AiProfile {
    fn default() -> Self {
        Self {
            name: "balanced".into(),
            aggression: 0.5,
            diplomacy_bias: 0.5,
            economic_focus: 0.5,
        }
    }
}
