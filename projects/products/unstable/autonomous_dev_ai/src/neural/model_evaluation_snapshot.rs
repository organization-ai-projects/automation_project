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

    pub fn load_scores_for_model(
        path: &str,
        model_name: &str,
    ) -> Result<Option<(f64, f64)>, String> {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("failed to read model evaluation snapshot '{path}': {e}"))?;

        if let Ok(single) = Self::from_json_str(&raw) {
            if single.model_name == model_name {
                return Ok(Some((single.offline_score, single.online_score)));
            }
            return Ok(None);
        }

        if let Ok(list) = serde_json::from_str::<Vec<ModelEvaluationSnapshot>>(&raw) {
            if let Some(found) = list.into_iter().find(|m| m.model_name == model_name) {
                return Ok(Some((found.offline_score, found.online_score)));
            }
            return Ok(None);
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct SnapshotCollection {
            models: Vec<ModelEvaluationSnapshot>,
        }

        if let Ok(collection) = serde_json::from_str::<SnapshotCollection>(&raw) {
            if let Some(found) = collection
                .models
                .into_iter()
                .find(|m| m.model_name == model_name)
            {
                return Ok(Some((found.offline_score, found.online_score)));
            }
            return Ok(None);
        }

        Err("invalid model evaluation snapshot JSON format".to_string())
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

    #[test]
    fn loads_scores_for_model_from_array() {
        let dir = std::env::temp_dir();
        let path = dir.join("autonomous_dev_ai_model_eval_array.json");
        let raw = r#"[
          {"model_name":"default-neural","offline_score":0.9,"online_score":0.8},
          {"model_name":"alt-neural","offline_score":0.7,"online_score":0.6}
        ]"#;
        std::fs::write(&path, raw).expect("write snapshot");

        let loaded = ModelEvaluationSnapshot::load_scores_for_model(
            path.to_str().expect("path utf8"),
            "alt-neural",
        )
        .expect("load scores");

        assert_eq!(loaded, Some((0.7, 0.6)));
        let _ = std::fs::remove_file(path);
    }
}
