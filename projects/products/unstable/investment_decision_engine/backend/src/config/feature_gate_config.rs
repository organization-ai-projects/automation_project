use serde::{Deserialize, Serialize};

use crate::config::EngineConfig;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureGateConfig {
    pub recommendation_output_enabled: bool,
    pub neural_assistance_enabled: bool,
}

impl FeatureGateConfig {
    pub fn from_config(config: &EngineConfig) -> Self {
        Self {
            recommendation_output_enabled: config.enable_recommendation_output,
            neural_assistance_enabled: config.enable_neural_assistance,
        }
    }

    pub fn is_recommendation_allowed(&self) -> bool {
        self.recommendation_output_enabled
    }

    pub fn is_neural_allowed(&self) -> bool {
        self.neural_assistance_enabled
    }
}

impl Default for FeatureGateConfig {
    fn default() -> Self {
        Self {
            recommendation_output_enabled: cfg!(feature = "recommendation_output"),
            neural_assistance_enabled: false,
        }
    }
}
