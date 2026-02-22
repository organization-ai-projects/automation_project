// projects/products/unstable/auto_manager_ai/src/config.rs

use crate::domain::Policy;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    EngineRequired,
    DeterministicFallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorIdentity {
    pub actor_id: String,
    pub actor_role: String,
}

/// Configuration for the automation manager
#[derive(Debug, Clone)]
pub struct Config {
    pub repo_path: PathBuf,
    pub output_dir: PathBuf,
    pub policy: Policy,
    pub run_mode: RunMode,
    pub actor: ActorIdentity,
}

impl Config {
    /// Create a new configuration
    pub fn new(repo_path: PathBuf, output_dir: PathBuf) -> Self {
        let run_mode = match std::env::var("AUTO_MANAGER_RUN_MODE") {
            Ok(value) if value.eq_ignore_ascii_case("deterministic_fallback") => {
                RunMode::DeterministicFallback
            }
            _ => RunMode::EngineRequired,
        };
        let actor = ActorIdentity {
            actor_id: std::env::var("AUTO_MANAGER_ACTOR_ID")
                .unwrap_or_else(|_| "auto_manager_ai".to_string()),
            actor_role: std::env::var("AUTO_MANAGER_ACTOR_ROLE")
                .unwrap_or_else(|_| "automation_service".to_string()),
        };
        Self {
            repo_path,
            output_dir,
            policy: Policy::default(),
            run_mode,
            actor,
        }
    }
}
