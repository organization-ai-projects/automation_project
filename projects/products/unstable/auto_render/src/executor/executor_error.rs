use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Capability denied for action '{action_id}': requires {capability}")]
    CapabilityDenied { action_id: String, capability: String },
    #[error("Action failed '{action_id}': {reason}")]
    ActionFailed { action_id: String, reason: String },
    #[error("Precondition failed for action '{action_id}': {condition}")]
    PreconditionFailed { action_id: String, condition: String },
    #[error("World state corrupted")]
    WorldCorrupted,
}
