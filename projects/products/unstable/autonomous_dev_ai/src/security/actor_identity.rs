//projects/products/unstable/autonomous_dev_ai/src/security/actor_identity.rs
use super::ActorRole;
use serde::{Deserialize, Serialize};

/// Identity of the actor driving autonomous actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorIdentity {
    pub id: String,
    pub roles: Vec<ActorRole>,
    pub run_id: String,
}

impl ActorIdentity {
    pub fn new(id: impl Into<String>, roles: Vec<ActorRole>, run_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            roles,
            run_id: run_id.into(),
        }
    }

    pub fn has_role(&self, role: &ActorRole) -> bool {
        self.roles.contains(role)
    }
}

impl Default for ActorIdentity {
    fn default() -> Self {
        Self::new(
            "autonomous_dev_ai",
            vec![ActorRole::Developer],
            "default_run",
        )
    }
}
