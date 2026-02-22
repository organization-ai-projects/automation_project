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
            ActorId::new("autonomous_dev_ai").expect("static actor id must be valid"),
            vec![ActorRole::Developer],
            RunId::new("default_run").expect("static run id must be valid"),
        )
    }
}

impl ActorIdentity {
    pub fn from_env_or_default() -> Self {
        let mut actor = Self::default();

        if let Ok(id) = std::env::var("AUTONOMOUS_ACTOR_ID")
            && let Some(parsed) = ActorId::new(id.trim())
        {
            actor.id = parsed;
        }

        if let Ok(run_id) = std::env::var("AUTONOMOUS_RUN_ID")
            && let Some(parsed) = RunId::new(run_id.trim())
        {
            actor.run_id = parsed;
        }

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
