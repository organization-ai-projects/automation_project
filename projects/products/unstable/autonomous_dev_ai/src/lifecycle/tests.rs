use super::*;
use crate::config::{AgentConfig, NeuralConfig, SymbolicConfig};
use crate::objectives::Objective;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static TEST_AUDIT_COUNTER: AtomicU64 = AtomicU64::new(0);

fn create_portable_test_audit_log_path() -> String {
    let id = TEST_AUDIT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "autonomous_dev_ai_lifecycle_test_{}_{}_{}.log",
        std::process::id(),
        std::thread::current().name().unwrap_or("thread"),
        id
    ));
    path.to_string_lossy().to_string()
}

fn create_test_config() -> AgentConfig {
    AgentConfig {
        agent_name: "test_agent".to_string(),
        execution_mode: "ci_bound".to_string(),
        max_iterations: 5,
        timeout_seconds: Some(60),
        objectives: vec![
            Objective::new("task_completion".to_string(), 1.0, true).with_threshold(0.5),
            Objective::new("policy_safety".to_string(), 1.0, true).with_threshold(1.0),
        ],
        symbolic: SymbolicConfig {
            strict_validation: true,
            deterministic: true,
        },
        neural: NeuralConfig {
            enabled: false,
            prefer_gpu: false,
            cpu_fallback: true,
            models: HashMap::new(),
        },
    }
}

#[test]
fn test_iteration_number() {
    let iter = IterationNumber::first();
    assert_eq!(iter.get(), 1);

    let next = iter.try_next().expect("next iteration should exist");
    assert_eq!(next.get(), 2);

    let max = MaxIterations::new(5).expect("valid max iterations");
    assert!(!iter.exceeds(max));

    let mut current = iter;
    for _ in 0..5 {
        current = current.try_next().expect("next iteration should exist");
    }
    assert!(current.exceeds(max));
}

#[test]
fn test_max_iterations() {
    let max = MaxIterations::new(10).expect("valid max iterations");
    assert_eq!(max.get(), 10);
    assert!(MaxIterations::new(0).is_none());
    assert_eq!(MaxIterations::default().get(), 10);
}

#[test]
fn test_step_index() {
    let step = StepIndex::zero();
    assert_eq!(step.get(), 0);
    assert_eq!(step.increment().get(), 1);
}

#[test]
fn test_execution_context_timeout() {
    let iter = IterationNumber::first();
    let ctx = ExecutionContext::new(iter, Duration::from_millis(100));

    assert!(!ctx.is_timed_out());
    std::thread::sleep(Duration::from_millis(120));
    assert!(ctx.is_timed_out());
    assert!(ctx.remaining_time().is_none());
}

#[test]
fn test_circuit_breaker() {
    let mut breaker = CircuitBreaker::new(2, 2, Duration::from_millis(100));

    assert_eq!(breaker.state(), CircuitState::Closed);
    assert!(breaker.should_allow_request());

    breaker.record_failure();
    assert_eq!(breaker.state(), CircuitState::Closed);

    breaker.record_failure();
    assert_eq!(breaker.state(), CircuitState::Open);
    assert!(!breaker.should_allow_request());

    std::thread::sleep(Duration::from_millis(120));

    assert!(breaker.should_allow_request());
    assert_eq!(breaker.state(), CircuitState::HalfOpen);

    breaker.record_success();
    breaker.record_success();
    assert_eq!(breaker.state(), CircuitState::Closed);
}

#[test]
fn test_retry_strategy() {
    let strategy = RetryStrategy::new(3);

    assert_eq!(strategy.max_attempts(), 3);

    let delay0 = strategy.delay_for_attempt(0).expect("delay for attempt 0");
    let delay1 = strategy.delay_for_attempt(1).expect("delay for attempt 1");
    let delay2 = strategy.delay_for_attempt(2).expect("delay for attempt 2");

    assert!(delay1 > delay0);
    assert!(delay2 > delay1);
    assert!(strategy.delay_for_attempt(3).is_none());
}

#[test]
fn test_lifecycle_manager_creation() {
    let config = create_test_config();
    let audit_log_path = create_portable_test_audit_log_path();
    let manager = LifecycleManager::new(config, &audit_log_path);

    assert_eq!(manager.current_state(), AgentState::Idle);
    assert_eq!(manager.current_iteration(), 1);

    let _ = std::fs::remove_file(audit_log_path);
}

#[test]
fn test_pr_number_extraction() {
    assert_eq!(
        LifecycleManager::extract_pr_number_from_goal("Fix issue #123"),
        Some("123".to_string())
    );

    assert_eq!(
        LifecycleManager::extract_pr_number_from_goal("Fix PR#456"),
        Some("456".to_string())
    );

    assert_eq!(
        LifecycleManager::extract_pr_number_from_goal("No PR here"),
        None
    );
}

#[test]
fn test_metrics_collector() {
    let metrics = MetricsCollector::new();

    metrics.record_iteration_start();
    metrics.record_tool_execution("test_tool", true, Duration::from_millis(100));
    metrics.record_iteration_success(Duration::from_millis(500));

    let snapshot = metrics.snapshot();

    assert_eq!(snapshot.iterations_total, 1);
    assert_eq!(snapshot.iterations_successful, 1);
    assert_eq!(snapshot.tool_executions_total, 1);
    assert_eq!(snapshot.tool_executions_failed, 0);
}
