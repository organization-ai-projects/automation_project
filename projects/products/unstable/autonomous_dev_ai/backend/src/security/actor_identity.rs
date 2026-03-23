// projects/products/unstable/autonomous_dev_ai/src/security/actor_identity.rs
use super::ActorRole;
use crate::ids::{ActorId, RunId};
use serde::{Deserialize, Serialize};

/// Identity of the actor driving autonomous actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorIdentity {
    pub id: ActorId,
    pub roles: Vec<ActorRole>,
    pub run_id: RunId,
}

impl ActorIdentity {
    pub fn new(id: ActorId, roles: Vec<ActorRole>, run_id: RunId) -> Self {
        Self { id, roles, run_id }
    }
}

impl Default for ActorIdentity {
    fn default() -> Self {
        Self::new(
            ActorId::default(),
            vec![ActorRole::Developer],
            RunId::default(),
        )
    }
}

impl ActorIdentity {
    pub fn from_env_or_default() -> Self {
        let mut actor = Self::default();

        if let Ok(raw_roles) = std::env::var("AUTONOMOUS_ACTOR_ROLES") {
            let mut parsed_roles = Vec::new();
            for role in raw_roles
                .split(',')
                .map(|r| r.trim().to_ascii_lowercase())
                .filter(|r| !r.is_empty())
            {
                let mapped = match role.as_str() {
                    "readonly" | "read_only" => Some(ActorRole::ReadOnly),
                    "developer" => Some(ActorRole::Developer),
                    "reviewer" => Some(ActorRole::Reviewer),
                    "operator" => Some(ActorRole::Operator),
                    "admin" => Some(ActorRole::Admin),
                    _ => None,
                };
                if let Some(role) = mapped {
                    parsed_roles.push(role);
                }
            }
            if !parsed_roles.is_empty() {
                actor.roles = parsed_roles;
            }
        }

        actor
    }
}
