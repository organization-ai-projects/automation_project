use crate::model::entity_id::EntityId;
use thiserror::Error;

/// Errors produced by the simulation engine.
#[derive(Debug, Error)]
pub enum SimError {
    #[error("unknown entity: {0}")]
    UnknownEntity(EntityId),
    #[error("simulation invariant violated: {0}")]
    InvariantViolation(String),
}
