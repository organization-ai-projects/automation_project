// projects/products/unstable/autonomous_dev_ai/src/neural_config.rs
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Neural configuration
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct NeuralConfig {
    pub enabled: bool,
    pub prefer_gpu: bool,
    pub cpu_fallback: bool,
    pub models: HashMap<String, String>,
}

impl Default for NeuralConfig {
    fn default() -> Self {
        let mut models = HashMap::new();
        models.insert("intent".to_string(), "intent_v1.bin".to_string());
        models.insert("codegen".to_string(), "codegen_v2.bin".to_string());
        models.insert("review".to_string(), "review_v1.bin".to_string());

        Self {
            enabled: true,
            prefer_gpu: true,
            cpu_fallback: true,
            models,
        }
    }
}
