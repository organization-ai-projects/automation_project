//! Lifecycle module surface (no business logic here).

mod errors;
mod manager;
mod metrics;
mod resilience;
mod types;

pub use errors::{LifecycleError, LifecycleResult, ResourceType};
pub use manager::LifecycleManager;
pub use metrics::{LifecycleMetrics, MetricsCollector};
pub use resilience::{
    ActionBoundary, Checkpoint, CircuitBreaker, CircuitState, CompensationKind, ResourceBudget,
    RetryStrategy, RollbackManager,
};
pub use types::{ExecutionContext, IterationNumber, MaxIterations, StepIndex};

#[cfg(test)]
mod tests;
