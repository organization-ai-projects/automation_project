// projects/products/unstable/autonomous_dev_ai/src/neural/model_evaluation_snapshot.rs
use serde::{Deserialize, Serialize};
use std::fs;

/// Offline/online evaluation scores used to gate rollout for a model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEvaluationSnapshot {
    pub model_name: String,
    pub offline_score: f64,
    pub online_score: f64,
}

impl ModelEvaluationSnapshot {
    pub fn from_json_str(raw: &str) -> Result<Self, String> {
        serde_json::from_str(raw).map_err(|e| format!("invalid model evaluation JSON: {e}"))
    }

    pub fn load_from_path(path: &str) -> Result<Self, String> {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("failed to read model evaluation snapshot '{path}': {e}"))?;
        Self::from_json_str(&raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_snapshot() {
        let raw = r#"{
            "model_name": "default-neural",
            "offline_score": 0.92,
            "online_score": 0.87
        }"#;
        let snapshot = ModelEvaluationSnapshot::from_json_str(raw).expect("valid snapshot");
        assert_eq!(snapshot.model_name, "default-neural");
        assert!((snapshot.offline_score - 0.92).abs() < f64::EPSILON);
        assert!((snapshot.online_score - 0.87).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_invalid_snapshot() {
        let raw = r#"{"model_name":"default-neural","offline_score":"bad"}"#;
        assert!(ModelEvaluationSnapshot::from_json_str(raw).is_err());
    }
}
