//! Enhanced lifecycle error handling.

use crate::error::AgentError;
use std::time::Duration;

#[derive(Debug)]
pub enum LifecycleError {
    Recoverable {
        iteration: usize,
        error: AgentError,
        retry_after: Option<Duration>,
    },
    Fatal {
        iteration: usize,
        error: AgentError,
        context: String,
    },
    ResourceExhausted {
        resource: ResourceType,
        limit: usize,
        current: usize,
    },
    Timeout {
        iteration: usize,
        elapsed: Duration,
        limit: Duration,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Iterations,
    Time,
    Memory,
    ToolExecutions,
}

impl LifecycleError {
    pub fn is_recoverable(&self) -> bool {
        matches!(self, Self::Recoverable { .. })
    }

    pub fn retry_delay(&self) -> Option<Duration> {
        match self {
            Self::Recoverable { retry_after, .. } => *retry_after,
            _ => None,
        }
    }

    pub fn as_agent_error(&self) -> Option<&AgentError> {
        match self {
            Self::Recoverable { error, .. } | Self::Fatal { error, .. } => Some(error),
            _ => None,
        }
    }
}

impl std::fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Recoverable {
                iteration, error, ..
            } => {
                write!(f, "Recoverable error at iteration {}: {}", iteration, error)
            }
            Self::Fatal {
                iteration,
                error,
                context,
            } => {
                write!(
                    f,
                    "Fatal error at iteration {}: {} ({})",
                    iteration, error, context
                )
            }
            Self::ResourceExhausted {
                resource,
                limit,
                current,
            } => {
                write!(f, "{resource:?} exhausted: {current}/{limit}")
            }
            Self::Timeout {
                iteration,
                elapsed,
                limit,
            } => {
                write!(
                    f,
                    "Timeout at iteration {}: {:?} > {:?}",
                    iteration, elapsed, limit
                )
            }
        }
    }
}

impl std::error::Error for LifecycleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.as_agent_error().map(|e| e as &dyn std::error::Error)
    }
}

impl From<LifecycleError> for AgentError {
    fn from(err: LifecycleError) -> Self {
        match err {
            LifecycleError::Recoverable { error, .. } | LifecycleError::Fatal { error, .. } => {
                error
            }
            LifecycleError::ResourceExhausted {
                resource,
                limit,
                current,
            } => AgentError::State(format!("{resource:?} exhausted: {current}/{limit}")),
            LifecycleError::Timeout {
                iteration,
                elapsed,
                limit,
            } => AgentError::State(format!(
                "Timeout at iteration {}: {:?} > {:?}",
                iteration, elapsed, limit
            )),
        }
    }
}

pub type LifecycleResult<T> = Result<T, LifecycleError>;
