//projects/products/unstable/autonomous_dev_ai/src/lifecycle/mod.rs
// Lifecycle module surface (no business logic here).
mod action_boundary;
mod checkpoint;
mod circuit_breaker;
mod circuit_state;
mod compensation_kind;
mod lifecycle_error;
mod lifecycle_manager;
mod lifecycle_metrics;
mod metrics_collector;
mod metrics_inner;
mod resource_budget;
mod resource_type;
mod retry_strategy;
mod rollback_manager;
mod tool_metrics;
mod types;

pub use action_boundary::ActionBoundary;
pub use checkpoint::Checkpoint;
pub use circuit_breaker::CircuitBreaker;
pub use circuit_state::CircuitState;
pub use compensation_kind::CompensationKind;
pub use lifecycle_error::{LifecycleError, LifecycleResult};
pub use lifecycle_manager::LifecycleManager;
pub use lifecycle_metrics::LifecycleMetrics;
pub use metrics_collector::MetricsCollector;
pub use metrics_inner::MetricsInner;
pub use resource_budget::ResourceBudget;
pub use resource_type::ResourceType;
pub use retry_strategy::RetryStrategy;
pub use rollback_manager::RollbackManager;
pub use tool_metrics::ToolMetrics;
pub use types::{ExecutionContext, IterationNumber, MaxIterations, StepIndex};
