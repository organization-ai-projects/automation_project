// projects/products/unstable/autonomous_dev_ai/src/security/actor_role.rs
use serde::{Deserialize, Serialize};

/// Roles an actor can hold.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorRole {
    ReadOnly,
    Developer,
    Reviewer,
    Operator,
    Admin,
}
