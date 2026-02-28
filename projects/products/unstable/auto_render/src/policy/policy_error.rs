use thiserror::Error;

#[derive(Error, Debug)]
pub enum PolicyError {
    #[error("Capability denied for action '{action_id}': requires {required:?}")]
    CapabilityDenied { action_id: String, required: String },
    #[error("Budget exceeded: {kind}")]
    BudgetExceeded { kind: String },
    #[error("Requires approval: {reason}")]
    RequiresApproval { reason: String },
    #[error("No policy snapshot available")]
    NoSnapshot,
}
