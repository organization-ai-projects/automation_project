// projects/products/unstable/autonomous_dev_ai/src/neural/rollout_state.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RolloutState {
    /// Model is registered but not yet evaluated.
    Pending,
    /// Model passed offline evaluation and is being canary-deployed.
    Canary,
    /// Model is fully deployed to production.
    Production,
    /// Model has been rolled back due to drift or failure.
    RolledBack,
}
