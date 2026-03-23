//! projects/products/unstable/autonomous_dev_ai/src/neural/model_evaluation_snapshot.rs
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
        common_json::from_str(raw).map_err(|e| format!("invalid model evaluation JSON: {e}"))
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

        if let Ok(list) = common_json::from_str::<Vec<ModelEvaluationSnapshot>>(&raw) {
            if let Some(found) = list.into_iter().find(|m| m.model_name == model_name) {
                return Ok(Some((found.offline_score, found.online_score)));
            }
            return Ok(None);
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct SnapshotCollection {
            models: Vec<ModelEvaluationSnapshot>,
        }

        if let Ok(collection) = common_json::from_str::<SnapshotCollection>(&raw) {
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
