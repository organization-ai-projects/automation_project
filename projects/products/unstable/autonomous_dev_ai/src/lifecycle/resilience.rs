//! Resilience patterns: circuit breaker, retry strategy, resource budgets,
//! checkpoint/restart, and rollback/compensation for the agent lifecycle.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    state: CircuitState,
    failure_count: usize,
    half_open_success_count: usize,
    failure_threshold: usize,
    success_threshold: usize,
    timeout: Duration,
    last_failure_time: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, success_threshold: usize, timeout: Duration) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            half_open_success_count: 0,
            failure_threshold,
            success_threshold,
            timeout,
            last_failure_time: None,
        }
    }

    pub fn should_allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed() > self.timeout {
                        tracing::info!("Circuit breaker transitioning to HalfOpen");
                        self.state = CircuitState::HalfOpen;
                        self.half_open_success_count = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                self.half_open_success_count = self.half_open_success_count.saturating_add(1);
                if self.half_open_success_count >= self.success_threshold {
                    tracing::info!("Circuit breaker transitioning to Closed");
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.half_open_success_count = 0;
                }
            }
            CircuitState::Open => {}
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count = self.failure_count.saturating_add(1);
        self.last_failure_time = Some(Instant::now());

        match self.state {
            CircuitState::Closed => {
                if self.failure_count >= self.failure_threshold {
                    tracing::warn!("Circuit breaker transitioning to Open");
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                tracing::warn!("Circuit breaker transitioning back to Open");
                self.state = CircuitState::Open;
                self.half_open_success_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    pub fn state(&self) -> CircuitState {
        self.state
    }
}

#[derive(Debug, Clone)]
pub struct RetryStrategy {
    max_attempts: usize,
    initial_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
}

impl RetryStrategy {
    pub fn new(max_attempts: usize) -> Self {
        Self {
            max_attempts,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }

    pub fn with_delays(mut self, initial: Duration, max: Duration) -> Self {
        self.initial_delay = initial;
        self.max_delay = max;
        self
    }

    pub fn delay_for_attempt(&self, attempt: usize) -> Option<Duration> {
        if attempt >= self.max_attempts {
            return None;
        }

        let delay_ms = self.initial_delay.as_millis() as f64 * self.multiplier.powi(attempt as i32);
        let delay = Duration::from_millis(delay_ms as u64);
        Some(delay.min(self.max_delay))
    }

    pub fn max_attempts(&self) -> usize {
        self.max_attempts
    }
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self::new(3)
    }
}

// ─── Resource Budget ──────────────────────────────────────────────────────────

/// Resource limits enforced during autonomous execution.
/// When any budget is exceeded the agent must transition to a fail-safe state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBudget {
    /// Maximum wall-clock runtime for the entire run.
    pub max_runtime: Duration,
    /// Maximum number of lifecycle iterations.
    pub max_iterations: usize,
    /// Maximum number of tool executions across the run.
    pub max_tool_executions: usize,
}

impl ResourceBudget {
    pub fn new(max_runtime: Duration, max_iterations: usize, max_tool_executions: usize) -> Self {
        Self {
            max_runtime,
            max_iterations,
            max_tool_executions,
        }
    }

    /// Check whether any budget limit has been breached.
    pub fn is_exceeded(
        &self,
        elapsed: Duration,
        iterations: usize,
        tool_executions: usize,
    ) -> Option<&'static str> {
        if elapsed >= self.max_runtime {
            return Some("runtime budget exceeded");
        }
        if iterations >= self.max_iterations {
            return Some("iteration budget exceeded");
        }
        if tool_executions >= self.max_tool_executions {
            return Some("tool execution budget exceeded");
        }
        None
    }
}

impl Default for ResourceBudget {
    fn default() -> Self {
        Self::new(Duration::from_secs(3600), 100, 500)
    }
}

// ─── Checkpoint / Restart ─────────────────────────────────────────────────────

/// Saved checkpoint that allows the agent to resume after a crash/restart.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub run_id: String,
    pub iteration: usize,
    pub state_label: String,
    pub timestamp_secs: u64,
}

impl Checkpoint {
    pub fn new(
        run_id: impl Into<String>,
        iteration: usize,
        state_label: impl Into<String>,
    ) -> Self {
        Self {
            run_id: run_id.into(),
            iteration,
            state_label: state_label.into(),
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Persist the checkpoint to a JSON file atomically (write-then-rename).
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let tmp = format!("{path}.tmp");
        std::fs::write(&tmp, &json)?;
        std::fs::rename(&tmp, path)
    }

    /// Load the latest checkpoint from a JSON file.
    pub fn load(path: &str) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

// ─── Rollback / Compensation ─────────────────────────────────────────────────

/// Describes whether an action can be undone and the compensation strategy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompensationKind {
    /// No rollback needed (read-only or idempotent).
    None,
    /// Can be reversed programmatically (e.g., delete the created branch).
    Reversible { description: String },
    /// Cannot be reversed; requires manual intervention.
    Irreversible { warning: String },
}

/// A recorded action boundary used by the rollback manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionBoundary {
    pub action_name: String,
    pub compensation: CompensationKind,
    pub timestamp_secs: u64,
}

/// Manages rollback/compensation paths for a run's action history.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RollbackManager {
    pub boundaries: Vec<ActionBoundary>,
}

impl RollbackManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an action and its compensation strategy.
    pub fn record(&mut self, action_name: impl Into<String>, compensation: CompensationKind) {
        self.boundaries.push(ActionBoundary {
            action_name: action_name.into(),
            compensation,
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });
    }

    /// Return all actions that require manual intervention (irreversible).
    pub fn irreversible_actions(&self) -> Vec<&ActionBoundary> {
        self.boundaries
            .iter()
            .filter(|b| matches!(b.compensation, CompensationKind::Irreversible { .. }))
            .collect()
    }

    /// Return all actions that can be automatically reversed.
    pub fn reversible_actions(&self) -> Vec<&ActionBoundary> {
        self.boundaries
            .iter()
            .filter(|b| matches!(b.compensation, CompensationKind::Reversible { .. }))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_budget_runtime_exceeded() {
        let budget = ResourceBudget::new(Duration::from_millis(1), 100, 500);
        std::thread::sleep(Duration::from_millis(5));
        assert!(
            budget.is_exceeded(Duration::from_millis(5), 0, 0).is_some(),
            "runtime budget should be exceeded"
        );
    }

    #[test]
    fn test_resource_budget_not_exceeded() {
        let budget = ResourceBudget::default();
        assert!(budget.is_exceeded(Duration::from_secs(10), 1, 10).is_none());
    }

    #[test]
    fn test_resource_budget_iteration_exceeded() {
        let budget = ResourceBudget::new(Duration::from_secs(3600), 5, 500);
        assert_eq!(
            budget.is_exceeded(Duration::ZERO, 5, 0),
            Some("iteration budget exceeded")
        );
    }

    #[test]
    fn test_checkpoint_save_and_load() {
        let mut tmp = std::env::temp_dir();
        tmp.push(format!(
            "checkpoint_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let path = tmp.to_string_lossy().into_owned();

        let cp = Checkpoint::new("run-42", 7, "EvaluateObjectives");
        cp.save(&path).unwrap();
        let loaded = Checkpoint::load(&path).unwrap();
        assert_eq!(loaded.run_id, "run-42");
        assert_eq!(loaded.iteration, 7);
        assert_eq!(loaded.state_label, "EvaluateObjectives");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_rollback_manager_classification() {
        let mut mgr = RollbackManager::new();
        mgr.record("read_file", CompensationKind::None);
        mgr.record(
            "git_commit",
            CompensationKind::Reversible {
                description: "git revert HEAD".to_string(),
            },
        );
        mgr.record(
            "deploy",
            CompensationKind::Irreversible {
                warning: "manual rollback required".to_string(),
            },
        );

        assert_eq!(mgr.reversible_actions().len(), 1);
        assert_eq!(mgr.irreversible_actions().len(), 1);
    }
}
