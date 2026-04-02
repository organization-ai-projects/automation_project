use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaitThesis {
    pub should_wait: bool,
    pub reason: String,
    pub recovery_probability: f64,
    pub estimated_recovery_horizon: Option<String>,
}

impl WaitThesis {
    pub fn hold_recommended(
        reason: impl Into<String>,
        recovery_probability: f64,
        horizon: Option<String>,
    ) -> Self {
        Self {
            should_wait: true,
            reason: reason.into(),
            recovery_probability,
            estimated_recovery_horizon: horizon,
        }
    }

    pub fn exit_recommended(reason: impl Into<String>) -> Self {
        Self {
            should_wait: false,
            reason: reason.into(),
            recovery_probability: 0.0,
            estimated_recovery_horizon: None,
        }
    }
}
