use std::path;

// projects/products/code_agent_sandbox/src/agents/agent_request.rs
use serde::Deserialize;

use crate::agents::default_max_iters;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRequest {
    /// L'intent humain (nocode) : "ajoute un endpoint", "refactor ce module", etc.
    pub intent: String,

    /// Max itérations
    #[serde(default = "default_max_iters")]
    pub max_iters: usize,

    /// Stratégie si tu veux forcer (sinon dispatcher décide)
    #[serde(default)]
    pub forced_strategy: Option<String>,

    /// Optionnel: le fichier principal à cibler (sinon l’IA peut se débrouiller en lisant src/)
    #[serde(default)]
    pub focus_file: Option<String>,

    /// Répertoire pour les modèles (ex: ./models)
    #[serde(default)]
    pub model_dir: Option<path::PathBuf>,

    /// Fichier pour le replay buffer (ex: ./replay.jsonl)
    #[serde(default)]
    pub replay_path: Option<path::PathBuf>,
}